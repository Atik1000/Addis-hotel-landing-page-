use crate::components::{pluralize, thousands, use_toast, Icon};
use crate::data::{find_hotel, find_room, Hotel, RoomType};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};
use std::time::Duration;

// ---------------------------------------------------------------------------
// Date helpers
//
// Only enough calendar maths to count nights and print a friendly date; the
// portal never needs timezone-aware handling because check-in dates are local
// to the hotel.
// ---------------------------------------------------------------------------

/// Days since 1970-01-01 for a proleptic Gregorian date (Howard Hinnant's algorithm).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn parse_iso(value: &str) -> Option<(i64, i64, i64)> {
    let mut parts = value.split('-');
    let y = parts.next()?.parse().ok()?;
    let m = parts.next()?.parse().ok()?;
    let d = parts.next()?.parse().ok()?;
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    Some((y, m, d))
}

/// Nights between two ISO dates, or `None` if either is unparseable or the
/// range is not at least one night.
fn nights_between(check_in: &str, check_out: &str) -> Option<u32> {
    let (y1, m1, d1) = parse_iso(check_in)?;
    let (y2, m2, d2) = parse_iso(check_out)?;
    let diff = days_from_civil(y2, m2, d2) - days_from_civil(y1, m1, d1);
    if diff >= 1 {
        Some(diff as u32)
    } else {
        None
    }
}

const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December",
];

/// `"2026-09-04"` -> `"4 September 2026"`.
pub fn pretty_date(value: &str) -> String {
    match parse_iso(value) {
        Some((y, m, d)) => format!("{} {} {}", d, MONTHS[(m - 1) as usize], y),
        None => value.to_string(),
    }
}

/// Deterministic-looking booking reference derived from the guest's details, so
/// the same submission always produces the same code within a session.
fn booking_reference(name: &str, contact: &str) -> String {
    let seed: u32 = name
        .bytes()
        .chain(contact.bytes())
        .fold(7u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
    format!("HA-{:06}", 100_000 + seed % 900_000)
}

const STEPS: [(&str, &str); 3] = [
    ("Your details", "user-check"),
    ("Your stay", "calendar"),
    ("Review & confirm", "check-circle"),
];

#[component]
pub fn ReservationFormPage() -> impl IntoView {
    let params = use_params_map();

    let resolved = move || {
        let hid = params.get().get("id").unwrap_or_default();
        let rid = params.get().get("room_id").unwrap_or_default();
        find_hotel(&hid).and_then(|h| find_room(h, &rid).map(|r| (h, r)))
    };

    view! {
        <Title text="Complete your reservation — Horn of Africa Hotel Portal" />
        {move || match resolved() {
            None => view! {
                <div class="mx-auto flex max-w-lg flex-col items-center gap-3 px-4 py-24 text-center">
                    <span class="flex h-16 w-16 items-center justify-center rounded-full bg-slate-100 text-slate-400">
                        <Icon name="bed" class="h-7 w-7" />
                    </span>
                    <h1 class="text-xl font-bold text-slate-900">"Room not found"</h1>
                    <p class="text-sm text-slate-500">"That room type is no longer listed for this hotel."</p>
                    <A href="/hotels" attr:class="mt-2 rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800">
                        "Browse hotels"
                    </A>
                </div>
            }.into_any(),
            Some((h, r)) => view! { <ReservationWizard hotel=h room=r /> }.into_any(),
        }}
    }
}

#[component]
fn ReservationWizard(hotel: &'static Hotel, room: &'static RoomType) -> impl IntoView {
    let h = hotel;
    let r = room;
    let navigate = use_navigate();
    let toast = use_toast();

    let step = RwSignal::new(0usize);
    let full_name = RwSignal::new(String::new());
    let contact = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let requests = RwSignal::new(String::new());
    let check_in = RwSignal::new("2026-09-04".to_string());
    let check_out = RwSignal::new("2026-09-06".to_string());
    let guests = RwSignal::new(2u32.min(r.guests).max(1));
    let rooms = RwSignal::new(1u32);
    let arrival = RwSignal::new("Afternoon (12:00 – 18:00)".to_string());
    let errors = RwSignal::new(Vec::<String>::new());
    let submitting = RwSignal::new(false);

    // ---- Derived pricing -------------------------------------------------
    let nights = Memo::new(move |_| nights_between(&check_in.get(), &check_out.get()).unwrap_or(0));
    let subtotal = Memo::new(move |_| r.price_per_night * nights.get() * rooms.get());
    // Regional tourism levy, quoted so the guest sees the real total up front.
    let taxes = Memo::new(move |_| subtotal.get() * 15 / 100);
    let total = Memo::new(move |_| subtotal.get() + taxes.get());

    let validate_step = move |s: usize| -> Vec<String> {
        let mut out = Vec::new();
        if s == 0 {
            if full_name.get().trim().len() < 3 {
                out.push("Enter the full name of the lead guest.".to_string());
            }
            let c = contact.get();
            let looks_like_phone = c.chars().filter(|ch| ch.is_ascii_digit()).count() >= 7;
            let looks_like_email = c.contains('@') && c.contains('.');
            if !looks_like_phone && !looks_like_email {
                out.push("Enter a phone number or an email address we can reach you on.".to_string());
            }
            let e = email.get();
            if !e.trim().is_empty() && (!e.contains('@') || !e.contains('.')) {
                out.push("That email address does not look right.".to_string());
            }
        }
        if s == 1 {
            if nights_between(&check_in.get(), &check_out.get()).is_none() {
                out.push("Check-out must be at least one night after check-in.".to_string());
            }
            if guests.get() > r.guests * rooms.get() {
                out.push(format!(
                    "{} sleeps up to {}. Add another room or reduce the party size.",
                    r.name,
                    pluralize(r.guests * rooms.get(), "guest")
                ));
            }
        }
        out
    };

    // The submit buttons sit at the bottom of a long form, so a failed validation
    // would otherwise only show a toast while the error list stayed off-screen.
    // Deferred by a tick so `<Show>` has mounted the banner before we look it up.
    let reveal_errors = move || {
        // Deferred a tick so `<Show>` has mounted the banner and the page has
        // reflowed around it before the browser works out where to scroll.
        // Instant, because the site-wide `scroll-behavior: smooth` animates
        // towards a position captured before that reflow and lands short. The
        // banner's own `scroll-mt-20` keeps it clear of the sticky header.
        set_timeout(
            || {
                let Some(el) = document().get_element_by_id("form-errors") else { return };
                let opts = web_sys::ScrollIntoViewOptions::new();
                opts.set_block(web_sys::ScrollLogicalPosition::Start);
                opts.set_behavior(web_sys::ScrollBehavior::Instant);
                el.scroll_into_view_with_scroll_into_view_options(&opts);
            },
            Duration::ZERO,
        );
    };

    let go_next = move |_| {
        let found = validate_step(step.get());
        errors.set(found.clone());
        if found.is_empty() {
            step.update(|s| *s = (*s + 1).min(2));
        } else {
            toast.error("Check the form", found[0].clone());
            reveal_errors();
        }
    };

    let go_back = move |_| {
        errors.set(Vec::new());
        step.update(|s| *s = s.saturating_sub(1));
    };

    let submit = {
        let navigate = navigate.clone();
        move |_: leptos::ev::MouseEvent| {
            let mut found = validate_step(0);
            found.extend(validate_step(1));
            errors.set(found.clone());
            if !found.is_empty() {
                toast.error("Something's missing", found[0].clone());
                step.set(if validate_step(0).is_empty() { 1 } else { 0 });
                reveal_errors();
                return;
            }
            submitting.set(true);
            let reference = booking_reference(&full_name.get(), &contact.get());
            toast.success("Reservation confirmed", format!("Your booking reference is {reference}."));
            navigate(&format!("/confirmation/{reference}"), Default::default());
        }
    };
    // `Show`'s fallback may render more than once, so the handler has to be
    // `Fn` rather than `FnOnce` — storing it makes it cheaply copyable.
    let submit = StoredValue::new(submit);

    let field = "w-full rounded-xl border border-slate-300 px-3.5 py-2.5 text-sm text-slate-800 transition-all duration-200 focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100";

    view! {
        <div class="mx-auto max-w-6xl px-4 py-6">
            <A
                href=format!("/hotels/{}", h.id)
                attr:class="group mb-4 inline-flex items-center gap-1.5 text-sm font-semibold text-slate-500 transition-colors hover:text-blue-700"
            >
                <Icon name="chevron-left" class="h-4 w-4 transition-transform duration-200 group-hover:-translate-x-0.5" />
                "Back to " {h.name}
            </A>

            // ---- Step indicator ---------------------------------------------
            <div class="mb-7 animate-fade-up">
                <div class="flex items-center">
                    {STEPS.iter().enumerate().map(|(i, (label, icon))| {
                        let is_last = i == STEPS.len() - 1;
                        view! {
                            <>
                                <button
                                    on:click=move |_| { if i < step.get() { step.set(i); } }
                                    class="flex shrink-0 items-center gap-2.5 text-left"
                                >
                                    <span class=move || format!(
                                        "flex h-10 w-10 items-center justify-center rounded-full border-2 transition-all duration-300 {}",
                                        if step.get() > i {
                                            "border-emerald-500 bg-emerald-500 text-white"
                                        } else if step.get() == i {
                                            "border-blue-700 bg-blue-700 text-white shadow-lg shadow-blue-700/30 scale-110"
                                        } else {
                                            "border-slate-300 bg-white text-slate-400"
                                        }
                                    )>
                                        {move || if step.get() > i {
                                            view! { <Icon name="check" class="h-4 w-4" /> }.into_any()
                                        } else {
                                            view! { <Icon name=*icon class="h-4 w-4" /> }.into_any()
                                        }}
                                    </span>
                                    <span class="hidden sm:block">
                                        <span class="block text-[11px] uppercase tracking-wide text-slate-400">{format!("Step {}", i + 1)}</span>
                                        <span class=move || format!(
                                            "block text-sm font-bold transition-colors {}",
                                            if step.get() >= i { "text-slate-900" } else { "text-slate-400" }
                                        )>{*label}</span>
                                    </span>
                                </button>
                                <Show when=move || !is_last>
                                    <span class="mx-3 h-0.5 flex-1 overflow-hidden rounded-full bg-slate-200">
                                        <span class=move || format!(
                                            "block h-full rounded-full bg-emerald-500 transition-all duration-500 {}",
                                            if step.get() > i { "w-full" } else { "w-0" }
                                        )></span>
                                    </span>
                                </Show>
                            </>
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_22rem]">
                // ================= FORM =================
                <div class="min-w-0">
                    <Show when=move || !errors.get().is_empty()>
                        <div id="form-errors" class="mb-4 scroll-mt-20 animate-fade-up rounded-xl border border-red-200 bg-red-50 p-4">
                            <p class="flex items-center gap-2 text-sm font-bold text-red-800">
                                <Icon name="alert" class="h-4 w-4" />
                                "Please fix the following"
                            </p>
                            <ul class="mt-2 flex flex-col gap-1 pl-6 text-sm text-red-700">
                                {move || errors.get().into_iter().map(|e| view! {
                                    <li class="list-disc">{e}</li>
                                }).collect_view()}
                            </ul>
                        </div>
                    </Show>

                    // ---- Step 1 ---------------------------------------------
                    <Show when=move || (step.get() == 0)>
                        <div class="animate-fade-up rounded-2xl border border-slate-200 bg-white p-6">
                            <h2 class="text-lg font-bold text-slate-900">"Who is the reservation for?"</h2>
                            <p class="mt-1 text-sm text-slate-500">
                                "The lead guest must present matching identification at check-in."
                            </p>

                            <div class="mt-5 flex flex-col gap-4">
                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                                        "Full name" <span class="text-red-500">"*"</span>
                                    </label>
                                    <div class="relative">
                                        <Icon name="user-check" class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
                                        <input
                                            type="text"
                                            placeholder="Ahmed Hassan"
                                            class=format!("{field} pl-9")
                                            prop:value=full_name
                                            on:input:target=move |ev| full_name.set(ev.target().value())
                                        />
                                    </div>
                                </div>

                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                                        "Phone number or email" <span class="text-red-500">"*"</span>
                                    </label>
                                    <div class="relative">
                                        <Icon name="phone" class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
                                        <input
                                            type="text"
                                            placeholder="0912 345 678"
                                            class=format!("{field} pl-9")
                                            prop:value=contact
                                            on:input:target=move |ev| contact.set(ev.target().value())
                                        />
                                    </div>
                                    <p class="mt-1.5 flex items-center gap-1.5 text-xs text-slate-400">
                                        <Icon name="info" class="h-3.5 w-3.5" />
                                        "Used to retrieve your booking later and by the hotel to reach you."
                                    </p>
                                </div>

                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                                        "Email " <span class="font-medium normal-case text-slate-400">"(optional)"</span>
                                    </label>
                                    <div class="relative">
                                        <Icon name="mail" class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
                                        <input
                                            type="email"
                                            placeholder="ahmed@example.com"
                                            class=format!("{field} pl-9")
                                            prop:value=email
                                            on:input:target=move |ev| email.set(ev.target().value())
                                        />
                                    </div>
                                </div>

                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                                        "Special requests " <span class="font-medium normal-case text-slate-400">"(optional)"</span>
                                    </label>
                                    <textarea
                                        rows="3"
                                        placeholder="High floor, extra pillows, cot for a toddler…"
                                        class=format!("{field} resize-none")
                                        prop:value=requests
                                        on:input:target=move |ev| requests.set(ev.target().value())
                                    ></textarea>
                                    <p class="mt-1.5 text-xs text-slate-400">
                                        "Requests are passed to the hotel but cannot be guaranteed."
                                    </p>
                                </div>
                            </div>
                        </div>
                    </Show>

                    // ---- Step 2 ---------------------------------------------
                    <Show when=move || (step.get() == 1)>
                        <div class="animate-fade-up rounded-2xl border border-slate-200 bg-white p-6">
                            <h2 class="text-lg font-bold text-slate-900">"When are you staying?"</h2>
                            <p class="mt-1 text-sm text-slate-500">
                                {format!("Check-in from {} · Check-out by {}", h.check_in_time, h.check_out_time)}
                            </p>

                            <div class="mt-5 grid gap-4 sm:grid-cols-2">
                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">"Check-in"</label>
                                    <input
                                        type="date"
                                        class=field
                                        prop:value=check_in
                                        on:input:target=move |ev| check_in.set(ev.target().value())
                                    />
                                </div>
                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">"Check-out"</label>
                                    <input
                                        type="date"
                                        class=field
                                        prop:value=check_out
                                        on:input:target=move |ev| check_out.set(ev.target().value())
                                    />
                                </div>
                            </div>

                            <div class="mt-3 flex items-center gap-2 rounded-xl bg-blue-50 px-4 py-3 text-sm text-blue-800">
                                <Icon name="moon" class="h-4 w-4 shrink-0" />
                                {move || match nights.get() {
                                    0 => "Choose a check-out date at least one night later.".to_string(),
                                    1 => "1 night".to_string(),
                                    n => format!("{n} nights"),
                                }}
                            </div>

                            <div class="mt-5 grid gap-4 sm:grid-cols-2">
                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">"Guests"</label>
                                    <NumberField value=guests min=1 max=16 icon="users" />
                                </div>
                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">"Rooms"</label>
                                    <NumberField value=rooms min=1 max=8 icon="bed" />
                                </div>
                            </div>

                            <div class="mt-5">
                                <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">"Estimated arrival time"</label>
                                <select
                                    class=field
                                    on:change:target=move |ev| arrival.set(ev.target().value())
                                >
                                    {[
                                        "Morning (before 12:00)",
                                        "Afternoon (12:00 – 18:00)",
                                        "Evening (18:00 – 22:00)",
                                        "Late night (after 22:00)",
                                    ].into_iter().map(|opt| view! {
                                        <option selected=move || (arrival.get() == opt)>{opt}</option>
                                    }).collect_view()}
                                </select>
                                <p class="mt-1.5 text-xs text-slate-400">
                                    "The front desk is staffed 24 hours — a late arrival will not lose your room."
                                </p>
                            </div>
                        </div>
                    </Show>

                    // ---- Step 3 ---------------------------------------------
                    <Show when=move || (step.get() == 2)>
                        <div class="flex animate-fade-up flex-col gap-4">
                            <div class="rounded-2xl border border-slate-200 bg-white p-6">
                                <h2 class="text-lg font-bold text-slate-900">"Check everything over"</h2>
                                <p class="mt-1 text-sm text-slate-500">"Nothing is charged now. You pay the hotel at check-in."</p>

                                <dl class="mt-5 divide-y divide-slate-100">
                                    <ReviewRow icon="user-check" label="Lead guest" value=Signal::derive(move || full_name.get()) />
                                    <ReviewRow icon="phone" label="Contact" value=Signal::derive(move || contact.get()) />
                                    <ReviewRow
                                        icon="mail"
                                        label="Email"
                                        value=Signal::derive(move || {
                                            let e = email.get();
                                            if e.trim().is_empty() { "Not provided".to_string() } else { e }
                                        })
                                    />
                                    <ReviewRow icon="calendar" label="Check-in" value=Signal::derive(move || pretty_date(&check_in.get())) />
                                    <ReviewRow icon="calendar" label="Check-out" value=Signal::derive(move || pretty_date(&check_out.get())) />
                                    <ReviewRow
                                        icon="users"
                                        label="Party"
                                        value=Signal::derive(move || format!(
                                            "{} {} · {} {}",
                                            guests.get(),
                                            if guests.get() == 1 { "guest" } else { "guests" },
                                            rooms.get(),
                                            if rooms.get() == 1 { "room" } else { "rooms" },
                                        ))
                                    />
                                    <ReviewRow icon="clock" label="Arrival" value=Signal::derive(move || arrival.get()) />
                                    <ReviewRow
                                        icon="message"
                                        label="Requests"
                                        value=Signal::derive(move || {
                                            let q = requests.get();
                                            if q.trim().is_empty() { "None".to_string() } else { q }
                                        })
                                    />
                                </dl>

                                <button
                                    on:click=move |_| step.set(0)
                                    class="mt-4 text-sm font-bold text-blue-700 hover:underline"
                                >
                                    "Edit details"
                                </button>
                            </div>

                            <div class="flex items-start gap-3 rounded-2xl border border-blue-200 bg-blue-50 p-4 text-sm text-blue-900">
                                <Icon name="wallet" class="mt-0.5 h-4.5 w-4.5 shrink-0" />
                                <div>
                                    <p class="font-bold">"Payment at the hotel"</p>
                                    <p class="mt-0.5 leading-relaxed">
                                        "No card is taken now. Present your booking reference at the front desk and settle the bill there in cash or by card."
                                    </p>
                                </div>
                            </div>

                            <div class=format!(
                                "flex items-start gap-3 rounded-2xl border p-4 text-sm {}",
                                if r.refundable { "border-emerald-200 bg-emerald-50 text-emerald-900" } else { "border-amber-200 bg-amber-50 text-amber-900" }
                            )>
                                <Icon name=if r.refundable { "check-circle" } else { "alert" } class="mt-0.5 h-4.5 w-4.5 shrink-0" />
                                <div>
                                    <p class="font-bold">
                                        {if r.refundable { "Free cancellation" } else { "Non-refundable rate" }}
                                    </p>
                                    <p class="mt-0.5 leading-relaxed">
                                        {if r.refundable {
                                            "Cancel free of charge up to 24 hours before check-in from My Reservation."
                                        } else {
                                            "This rate cannot be refunded if you cancel or do not arrive."
                                        }}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </Show>

                    // ---- Navigation ---------------------------------------------
                    <div class="mt-5 flex items-center justify-between gap-3">
                        <button
                            on:click=go_back
                            disabled=move || (step.get() == 0)
                            class="flex items-center gap-1.5 rounded-xl border border-slate-300 px-5 py-3 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-40"
                        >
                            <Icon name="chevron-left" class="h-4 w-4" />
                            "Back"
                        </button>

                        <Show
                            when=move || (step.get() < 2)
                            fallback=move || view! {
                                <button
                                    on:click=move |ev| submit.with_value(|f| f(ev))
                                    disabled=submitting
                                    class="sheen flex items-center gap-2 rounded-xl bg-blue-700 px-7 py-3 text-sm font-bold text-white shadow-lg shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-xl active:scale-[0.98] disabled:opacity-70"
                                >
                                    <Show
                                        when=move || submitting.get()
                                        fallback=|| view! { <Icon name="check-circle" class="h-4 w-4" /> }
                                    >
                                        <span class="animate-spin-slow"><Icon name="loader" class="h-4 w-4" /></span>
                                    </Show>
                                    "Confirm reservation"
                                </button>
                            }
                        >
                            <button
                                on:click=go_next
                                class="sheen flex items-center gap-2 rounded-xl bg-blue-700 px-7 py-3 text-sm font-bold text-white shadow-lg shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-xl active:scale-[0.98]"
                            >
                                "Continue"
                                <Icon name="arrow-right" class="h-4 w-4" />
                            </button>
                        </Show>
                    </div>
                </div>

                // ================= SUMMARY =================
                <aside class="lg:sticky lg:top-24 lg:h-fit">
                    <div class="animate-slide-in-right overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-lg shadow-slate-900/5">
                        <div class="skeleton relative h-32 overflow-hidden">
                            <img src=h.image alt=h.name class="h-full w-full object-cover" />
                            <span class="absolute inset-0 bg-gradient-to-t from-slate-950/80 to-transparent"></span>
                            <span class="absolute inset-x-4 bottom-3 text-white">
                                <span class="block text-sm font-bold">{h.name}</span>
                                <span class="flex items-center gap-1 text-xs text-slate-300">
                                    <Icon name="map-pin" class="h-3 w-3" />
                                    {format!("{}, {}", h.area, h.city)}
                                </span>
                            </span>
                        </div>

                        <div class="p-5">
                            <div class="flex items-start gap-3 border-b border-slate-100 pb-4">
                                <img src=r.image alt=r.name class="h-14 w-14 shrink-0 rounded-xl object-cover" />
                                <div class="min-w-0">
                                    <p class="text-sm font-bold text-slate-900">{r.name}</p>
                                    <p class="text-xs text-slate-500">{r.beds}</p>
                                    <p class="text-xs text-slate-500">{format!("Sleeps {} · {} m²", r.guests, r.size_sqm)}</p>
                                </div>
                            </div>

                            <dl class="flex flex-col gap-2 py-4 text-sm">
                                <div class="flex justify-between gap-3">
                                    <dt class="text-slate-500">"Check-in"</dt>
                                    <dd class="font-semibold text-slate-800">{move || pretty_date(&check_in.get())}</dd>
                                </div>
                                <div class="flex justify-between gap-3">
                                    <dt class="text-slate-500">"Check-out"</dt>
                                    <dd class="font-semibold text-slate-800">{move || pretty_date(&check_out.get())}</dd>
                                </div>
                                <div class="flex justify-between gap-3">
                                    <dt class="text-slate-500">"Guests"</dt>
                                    <dd class="font-semibold text-slate-800">
                                        {move || format!("{} · {}", pluralize(guests.get(), "guest"), pluralize(rooms.get(), "room"))}
                                    </dd>
                                </div>
                            </dl>

                            <dl class="flex flex-col gap-2 border-t border-slate-100 py-4 text-sm">
                                <div class="flex justify-between gap-3">
                                    <dt class="text-slate-500">
                                        {move || format!(
                                            "ETB {} × {} × {}",
                                            thousands(r.price_per_night),
                                            pluralize(nights.get(), "night"),
                                            pluralize(rooms.get(), "room")
                                        )}
                                    </dt>
                                    <dd class="shrink-0 font-semibold text-slate-800 tabular-nums">
                                        {move || thousands(subtotal.get())}
                                    </dd>
                                </div>
                                <div class="flex justify-between gap-3">
                                    <dt class="flex items-center gap-1 text-slate-500">
                                        "Taxes & tourism levy"
                                        <span class="text-xs text-slate-400">"(15%)"</span>
                                    </dt>
                                    <dd class="shrink-0 font-semibold text-slate-800 tabular-nums">
                                        {move || thousands(taxes.get())}
                                    </dd>
                                </div>
                                <div class="flex justify-between gap-3">
                                    <dt class="text-slate-500">"Booking fee"</dt>
                                    <dd class="shrink-0 font-bold text-emerald-600">"FREE"</dd>
                                </div>
                            </dl>

                            <div class="flex items-end justify-between gap-3 border-t border-slate-200 pt-4">
                                <span>
                                    <span class="block text-sm font-bold text-slate-800">"Total"</span>
                                    <span class="block text-xs text-slate-400">"Payable at the hotel"</span>
                                </span>
                                <span class="text-2xl font-extrabold tracking-tight text-slate-900 tabular-nums">
                                    {move || format!("ETB {}", thousands(total.get()))}
                                </span>
                            </div>

                            <ul class="mt-4 flex flex-col gap-2 border-t border-slate-100 pt-4 text-xs text-slate-600">
                                {[
                                    ("shield-check", "No booking fees, ever"),
                                    ("wallet", "Nothing charged online"),
                                    ("check-circle", "Confirmed instantly"),
                                ].into_iter().map(|(icon, label)| view! {
                                    <li class="flex items-center gap-2">
                                        <Icon name=icon class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                        {label}
                                    </li>
                                }).collect_view()}
                            </ul>
                        </div>
                    </div>
                </aside>
            </div>
        </div>
    }
}

#[component]
fn NumberField(value: RwSignal<u32>, min: u32, max: u32, icon: &'static str) -> impl IntoView {
    let btn = "flex h-9 w-9 items-center justify-center rounded-lg border border-slate-300 text-slate-600 transition-all duration-150 hover:border-blue-400 hover:text-blue-700 active:scale-90 disabled:cursor-not-allowed disabled:opacity-35";
    view! {
        <div class="flex items-center justify-between gap-3 rounded-xl border border-slate-300 px-3 py-2">
            <span class="flex items-center gap-2 text-sm font-semibold text-slate-800">
                <Icon name=icon class="h-4 w-4 text-slate-400" />
                <span class="tabular-nums">{move || value.get()}</span>
            </span>
            <span class="flex items-center gap-2">
                <button
                    class=btn
                    disabled=move || (value.get() <= min)
                    on:click=move |_| value.update(|v| *v = v.saturating_sub(1).max(min))
                >
                    <Icon name="minus" class="h-3.5 w-3.5" />
                </button>
                <button
                    class=btn
                    disabled=move || (value.get() >= max)
                    on:click=move |_| value.update(|v| *v = (*v + 1).min(max))
                >
                    <Icon name="plus" class="h-3.5 w-3.5" />
                </button>
            </span>
        </div>
    }
}

#[component]
fn ReviewRow(
    icon: &'static str,
    label: &'static str,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="flex items-start justify-between gap-4 py-3">
            <dt class="flex shrink-0 items-center gap-2 text-sm text-slate-500">
                <Icon name=icon class="h-4 w-4 text-slate-400" />
                {label}
            </dt>
            <dd class="text-right text-sm font-semibold text-slate-800">
                {move || {
                    let v = value.get();
                    if v.trim().is_empty() { "—".to_string() } else { v }
                }}
            </dd>
        </div>
    }
}
