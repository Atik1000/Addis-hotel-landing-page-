//! The booking wizard, backed by `/reservations/public/`.
//!
//! Three steps: who is staying, when, then a review. Two live endpoints do the
//! real work:
//!
//! * `POST /reservations/public/quote/` prices the stay as the dates and party
//!   size change, so the guest sees the actual total — including the property's
//!   tax rate — before committing. It also validates the range: an impossible
//!   or already-booked date pair fails here rather than at submit.
//! * `POST /reservations/public/book/` creates the reservation. No account is
//!   needed; the email or phone entered here is what retrieves it later.
//!
//! The dates picked in the search widget ride through the query string, so the
//! wizard opens on the range the guest actually chose rather than resetting to
//! a default they then have to re-enter.
//!
//! Nothing is charged — the guest pays the hotel on arrival.

use crate::api::{
    create_booking, get_hotel_detail, list_hotel_rooms, money, money_round, pretty_date,
    BookingRequest, HotelDetail, Quote, RoomSummary,
};
use crate::components::{pluralize, Icon};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map, use_query_map};

/// Days since 1970-01-01 (Howard Hinnant's `days_from_civil`).
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
    let mut it = value.split('-');
    Some((
        it.next()?.parse().ok()?,
        it.next()?.parse().ok()?,
        it.next()?.parse().ok()?,
    ))
}

/// Whole nights between two ISO dates. `None` when the range is not at least
/// one night, which is exactly what the API rejects.
fn nights_between(check_in: &str, check_out: &str) -> Option<u32> {
    let (a, b) = (parse_iso(check_in)?, parse_iso(check_out)?);
    let diff = days_from_civil(b.0, b.1, b.2) - days_from_civil(a.0, a.1, a.2);
    (diff >= 1).then_some(diff as u32)
}

/// Today, from the browser clock on the client and the system clock on the
/// server. Both sides must produce a real date: leaving it blank during SSR
/// makes the price quote fail before hydration can fill it in.
fn today_iso() -> String {
    #[cfg(feature = "hydrate")]
    {
        let now = js_sys::Date::new_0();
        format!(
            "{:04}-{:02}-{:02}",
            now.get_full_year(),
            now.get_month() + 1,
            now.get_date()
        )
    }
    #[cfg(not(feature = "hydrate"))]
    {
        // UTC is close enough to seed a date picker, and the guest can change it.
        let days = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() / 86_400)
            .unwrap_or(0) as i64;
        let (y, m, d) = civil_from_days(days);
        format!("{y:04}-{m:02}-{d:02}")
    }
}

/// Inverse of [`days_from_civil`] (Howard Hinnant's `civil_from_days`).
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// Moves an ISO date by whole days.
fn plus_days(iso: &str, days: i64) -> String {
    let Some((y, m, d)) = parse_iso(iso) else {
        return String::new();
    };
    let (year, month, day) = civil_from_days(days_from_civil(y, m, d) + days);
    format!("{year:04}-{month:02}-{day:02}")
}

#[component]
pub fn ReservationFormPage() -> impl IntoView {
    let params = use_params_map();
    let hotel_id = move || params.get().get("id").unwrap_or_default();
    let room_id = move || params.get().get("room_id").unwrap_or_default();

    let hotel = Resource::new(hotel_id, |id| async move { get_hotel_detail(id).await });
    let rooms = Resource::new(hotel_id, |id| async move {
        list_hotel_rooms(id, None, None).await
    });

    view! {
        <Title text="Complete your reservation — Horn of Africa Hotel Portal" />
        <Suspense fallback=|| view! {
            <div class="mx-auto max-w-6xl px-4 py-10">
                <div class="skeleton h-96 rounded-2xl"></div>
            </div>
        }>
            {move || Suspend::new(async move {
                let wanted = room_id();
                let hotel = hotel.await;
                let room = rooms
                    .await
                    .ok()
                    .and_then(|list| list.into_iter().find(|r| r.id.to_string() == wanted));

                match (hotel, room) {
                    (Ok(h), Some(r)) => view! { <ReservationWizard hotel=h room=r /> }.into_any(),
                    _ => view! {
                        <div class="mx-auto flex max-w-lg flex-col items-center gap-3 px-4 py-24 text-center">
                            <span class="flex h-16 w-16 items-center justify-center rounded-full bg-slate-100 text-slate-400">
                                <Icon name="bed" class="h-7 w-7" />
                            </span>
                            <h1 class="text-xl font-bold text-ink">"Room not found"</h1>
                            <p class="text-sm text-slate-500">"That room is no longer listed for this hotel."</p>
                            <A href="/hotels" attr:class="mt-2 rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800">
                                "Browse hotels"
                            </A>
                        </div>
                    }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn ReservationWizard(hotel: HotelDetail, room: RoomSummary) -> impl IntoView {
    let navigate = use_navigate();
    let query = use_query_map();
    let nav = StoredValue::new(navigate);

    let hotel_id = hotel.id;
    let room_id = room.id;
    let hotel_name = hotel.name.clone();
    let room_name = room.name.clone();
    let room_number = room.room_number.clone();
    let capacity = room.guest_capacity.max(1);
    let currency = hotel.currency_code().to_string();
    let cur = StoredValue::new(currency.clone());
    let room_label = StoredValue::new(room.name.clone());
    let nightly = room.price();
    let room_image = crate::images::room_image(
        room.id,
        room.primary_image.as_deref(),
        room.room_type.as_deref(),
        500,
    );
    let bed_summary = room.bed_summary();
    let breakfast = room.breakfast_included;
    let policies = hotel.policies.clone().unwrap_or_default();
    let checkin_time = policies.checkin_display();
    let checkout_time = policies.checkout_display();
    let extra_beds_allowed = policies.extrabed_available.unwrap_or(false);
    let pets_allowed = policies.pet_allowed.unwrap_or(false);

    // Seeded from tomorrow so the API's "check-in not in the past" rule passes,
    // then overridden by whatever the guest searched for. A range that has since
    // gone stale — a check-in now in the past, or an end that is not after the
    // start — falls back rather than opening the form on dates the API will
    // reject.
    let start = today_iso();
    let fallback_in = plus_days(&start, 1);
    let fallback_out = plus_days(&start, 3);

    let wanted = query.get_untracked();
    let usable = |raw: Option<String>| -> Option<String> {
        let v = raw?;
        parse_iso(&v).is_some().then_some(v)
    };
    let (default_in, default_out) = match (
        usable(wanted.get("check_in")),
        usable(wanted.get("check_out")),
    ) {
        (Some(ci), Some(co))
            if nights_between(&ci, &co).is_some_and(|n| n > 0) && ci >= start =>
        {
            (ci, co)
        }
        _ => (fallback_in, fallback_out),
    };

    let step = RwSignal::new(0usize);
    let first_name = RwSignal::new(String::new());
    let last_name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let phone = RwSignal::new(String::new());
    let requests = RwSignal::new(String::new());
    let check_in = RwSignal::new(default_in);
    let check_out = RwSignal::new(default_out);
    let guests = RwSignal::new(2u32.min(capacity).max(1));
    let extra_bed = RwSignal::new(0u32);
    let pets = RwSignal::new(false);
    let babies = RwSignal::new(false);
    let errors = RwSignal::new(Vec::<String>::new());
    let submitting = RwSignal::new(false);

    let nights = Memo::new(move |_| nights_between(&check_in.get(), &check_out.get()).unwrap_or(0));

    // The API prices the stay. Re-quoted whenever anything that affects the
    // total changes, and only once the range is valid.
    let quote = Resource::new(
        move || {
            (
                room_id,
                check_in.get(),
                check_out.get(),
                guests.get(),
                extra_bed.get(),
                pets.get(),
            )
        },
        |(room_id, ci, co, g, eb, pet)| async move {
            // A range that is not yet a whole night is an unfinished form, not an
            // error — return nothing rather than shouting at the guest.
            if nights_between(&ci, &co).is_none() {
                return Ok(None);
            }
            crate::api::get_quote(room_id, ci, co, g, eb, pet)
                .await
                .map(Some)
        },
    );

    let validate_step = move |s: usize| -> Vec<String> {
        let mut out = Vec::new();
        if s == 0 {
            if first_name.get().trim().len() < 2 {
                out.push("Enter the lead guest's first name.".to_string());
            }
            if last_name.get().trim().len() < 2 {
                out.push("Enter the lead guest's last name.".to_string());
            }
            let e = email.get();
            let p = phone.get();
            let has_email = e.contains('@') && e.contains('.');
            let has_phone = p.chars().filter(|c| c.is_ascii_digit()).count() >= 7;
            if !has_email && !has_phone {
                out.push(
                    "Enter an email address or a phone number — it is how you retrieve the booking."
                        .to_string(),
                );
            }
            if !e.trim().is_empty() && !has_email {
                out.push("That email address does not look right.".to_string());
            }
        }
        if s == 1 {
            if nights_between(&check_in.get(), &check_out.get()).is_none() {
                out.push("Check-out must be at least one night after check-in.".to_string());
            }
            if guests.get() > capacity {
                out.push(format!(
                    "{} sleeps up to {}.",
                    room_label.get_value(),
                    pluralize(capacity, "guest"),
                ));
            }
            // Surface whatever the quote endpoint objected to, so the guest is
            // not told the dates are fine and then refused at submit.
            if let Some(Err(e)) = quote.get() {
                out.push(e.to_string());
            }
        }
        out
    };

    // The submit button sits at the bottom of a long form, so a failed
    // validation would otherwise only show a toast while the error list stayed
    // off-screen.
    let reveal_errors = move || {
        #[cfg(feature = "hydrate")]
        {
            leptos::prelude::set_timeout(
                || {
                    let Some(el) = document().get_element_by_id("form-errors") else {
                        return;
                    };
                    let opts = web_sys::ScrollIntoViewOptions::new();
                    opts.set_block(web_sys::ScrollLogicalPosition::Start);
                    opts.set_behavior(web_sys::ScrollBehavior::Instant);
                    el.scroll_into_view_with_scroll_into_view_options(&opts);
                },
                std::time::Duration::ZERO,
            );
        }
    };

    let go_next = move |_| {
        let found = validate_step(step.get());
        errors.set(found.clone());
        if found.is_empty() {
            step.update(|s| *s = (*s + 1).min(2));
        } else {
            reveal_errors();
        }
    };

    let go_back = move |_| {
        errors.set(Vec::new());
        step.update(|s| *s = s.saturating_sub(1));
    };

    let submit = move |_: leptos::ev::MouseEvent| {
        if submitting.get() {
            return;
        }
        let mut found = validate_step(0);
        found.extend(validate_step(1));
        errors.set(found.clone());
        if !found.is_empty() {
            step.set(if validate_step(0).is_empty() { 1 } else { 0 });
            reveal_errors();
            return;
        }
        submitting.set(true);

        let req = BookingRequest {
            organization_id: hotel_id,
            room_id,
            check_in_date: check_in.get(),
            check_out_date: check_out.get(),
            guest_count: guests.get(),
            extra_bed: extra_bed.get(),
            pet_presence: pets.get(),
            baby_presence: babies.get(),
            guest_first_name: first_name.get().trim().to_string(),
            guest_last_name: last_name.get().trim().to_string(),
            guest_email: email.get().trim().to_string(),
            guest_phone: phone.get().trim().to_string(),
            guest_note: requests.get(),
        };

        leptos::task::spawn_local(async move {
            match create_booking(req).await {
                Ok(res) => {
                    // Cache it so the confirmation page — and "My reservations"
                    // — can show the booking without a retrieval code.
                    crate::store::remember(&res);
                    let reference = res.reference();
                    nav.with_value(|n| {
                        n(&format!("/confirmation/{reference}"), Default::default())
                    });
                }
                Err(e) => {
                    submitting.set(false);
                    errors.set(vec![e.to_string()]);
                    reveal_errors();
                }
            }
        });
    };
    let submit = StoredValue::new(submit);

    let field = "w-full rounded-xl border border-slate-300 px-3.5 py-2.5 text-sm text-slate-800 transition-all duration-200 focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100";

    let back_href = format!("/hotels/{hotel_id}");
    let back_label = hotel_name.clone();

    view! {
        <div class="mx-auto max-w-6xl px-4 py-6">
            <A
                href=back_href
                attr:class="group mb-4 inline-flex items-center gap-1.5 text-sm font-semibold text-slate-500 transition-colors hover:text-blue-700"
            >
                <Icon name="chevron-left" class="h-4 w-4 transition-transform duration-200 group-hover:-translate-x-0.5" />
                "Back to " {back_label}
            </A>

            // ---- Step indicator ------------------------------------------
            <div class="mb-6 grid grid-cols-3 gap-2">
                {["Your details", "Your stay", "Review"].into_iter().enumerate().map(|(i, label)| view! {
                    <button
                        on:click=move |_| { if i < step.get() { step.set(i); } }
                        class="text-left"
                    >
                        <span class="flex items-center gap-2">
                            <span class=move || format!(
                                "flex h-7 w-7 shrink-0 items-center justify-center rounded-full text-xs font-bold transition-colors {}",
                                if step.get() > i {
                                    "bg-emerald-600 text-white"
                                } else if step.get() == i {
                                    "bg-blue-700 text-white"
                                } else {
                                    "bg-slate-200 text-slate-500"
                                }
                            )>
                                {move || if step.get() > i {
                                    view! { <Icon name="check" class="h-3.5 w-3.5" /> }.into_any()
                                } else {
                                    view! { {i + 1} }.into_any()
                                }}
                            </span>
                            <span class="min-w-0">
                                <span class="block text-[11px] uppercase tracking-wide text-slate-400">{format!("Step {}", i + 1)}</span>
                                <span class=move || format!(
                                    "block truncate text-sm font-bold transition-colors {}",
                                    if step.get() >= i { "text-ink" } else { "text-slate-400" }
                                )>
                                    {label}
                                </span>
                            </span>
                        </span>
                        <span class="mt-2 block h-1 overflow-hidden rounded-full bg-slate-200">
                            <span class=move || format!(
                                "block h-full rounded-full bg-blue-700 transition-all duration-500 {}",
                                if step.get() > i { "w-full" } else { "w-0" }
                            )></span>
                        </span>
                    </button>
                }).collect_view()}
            </div>

            <div class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_20rem]">
                // ================= FORM =================
                <div class="min-w-0">
                    <Show when=move || !errors.get().is_empty()>
                        <div id="form-errors" class="mb-5 scroll-mt-20 rounded-xl border border-red-200 bg-red-50 p-4">
                            <p class="flex items-center gap-2 text-sm font-bold text-red-800">
                                <Icon name="alert" class="h-4 w-4 shrink-0" />
                                "Please fix the following"
                            </p>
                            <ul class="mt-2 flex list-inside list-disc flex-col gap-1 text-sm text-red-700">
                                {move || errors.get().into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
                            </ul>
                        </div>
                    </Show>

                    // ---- Step 1: guest details ---------------------------
                    <Show when=move || (step.get() == 0)>
                        <section class="animate-fade-up rounded-2xl border border-slate-200 bg-white p-5">
                            <h2 class="text-lg font-bold text-ink">"Who is the reservation for?"</h2>
                            <p class="mt-1 text-sm text-slate-500">
                                "No account needed. We use these details to hold the room and to let you retrieve the booking later."
                            </p>

                            <div class="mt-4 grid gap-4 sm:grid-cols-2">
                                <Field label="First name" value=first_name class=field placeholder="Marta" />
                                <Field label="Last name" value=last_name class=field placeholder="Bekele" />
                            </div>

                            <div class="mt-4 grid gap-4 sm:grid-cols-2">
                                <Field label="Email address" value=email class=field kind="email" placeholder="you@example.com" />
                                <Field label="Phone number" value=phone class=field kind="tel" placeholder="+251 …" />
                            </div>
                            <p class="mt-1.5 text-xs text-slate-400">
                                "Give at least one. Your booking reference and the retrieval code are sent there."
                            </p>

                            <div class="mt-4">
                                <label class="mb-1.5 block text-sm font-semibold text-slate-700">"Special requests"</label>
                                <textarea
                                    rows="3"
                                    placeholder="Late arrival, high floor, airport pickup…"
                                    class=field
                                    prop:value=requests
                                    on:input:target=move |ev| requests.set(ev.target().value())
                                ></textarea>
                                <p class="mt-1.5 text-xs text-slate-400">
                                    "Requests are passed to the hotel but are not guaranteed."
                                </p>
                            </div>
                        </section>
                    </Show>

                    // ---- Step 2: the stay --------------------------------
                    <Show when=move || (step.get() == 1)>
                        <section class="animate-fade-up rounded-2xl border border-slate-200 bg-white p-5">
                            <h2 class="text-lg font-bold text-ink">"When are you staying?"</h2>
                            <p class="mt-1 text-sm text-slate-500">
                                {format!("Check-in from {checkin_time}, check-out by {checkout_time}.")}
                            </p>

                            <div class="mt-4 grid gap-4 sm:grid-cols-2">
                                <div>
                                    <label class="mb-1.5 block text-sm font-semibold text-slate-700">"Check-in"</label>
                                    <input type="date" class=field prop:value=check_in
                                        on:input:target=move |ev| check_in.set(ev.target().value()) />
                                </div>
                                <div>
                                    <label class="mb-1.5 block text-sm font-semibold text-slate-700">"Check-out"</label>
                                    <input type="date" class=field prop:value=check_out
                                        on:input:target=move |ev| check_out.set(ev.target().value()) />
                                </div>
                            </div>

                            <p class="mt-2 text-sm font-semibold text-slate-600">
                                {move || match nights.get() {
                                    0 => "Pick a valid date range".to_string(),
                                    n => pluralize(n, "night"),
                                }}
                            </p>

                            <div class="mt-5 grid gap-4 sm:grid-cols-2">
                                <div>
                                    <label class="mb-1.5 block text-sm font-semibold text-slate-700">"Guests"</label>
                                    <NumberField value=guests min=1 max=capacity icon="users" />
                                    <p class="mt-1.5 text-xs text-slate-400">
                                        {format!("This room sleeps up to {}.", pluralize(capacity, "guest"))}
                                    </p>
                                </div>
                                <Show when=move || extra_beds_allowed>
                                    <div>
                                        <label class="mb-1.5 block text-sm font-semibold text-slate-700">"Extra beds"</label>
                                        <NumberField value=extra_bed min=0 max=3 icon="bed" />
                                        <p class="mt-1.5 text-xs text-slate-400">"Charged per night by the hotel."</p>
                                    </div>
                                </Show>
                            </div>

                            <div class="mt-5 flex flex-col gap-2">
                                <Show when=move || pets_allowed>
                                    <Check label="I am bringing a pet" hint="The hotel may add a pet charge" value=pets />
                                </Show>
                                <Check label="I am travelling with an infant" hint="A cot may be arranged with the hotel" value=babies />
                            </div>
                        </section>
                    </Show>

                    // ---- Step 3: review ----------------------------------
                    <Show when=move || (step.get() == 2)>
                        <section class="animate-fade-up rounded-2xl border border-slate-200 bg-white p-5">
                            <h2 class="text-lg font-bold text-ink">"Check everything over"</h2>
                            <p class="mt-1 text-sm text-slate-500">
                                "Nothing is charged now — you pay the hotel when you arrive."
                            </p>

                            <dl class="mt-4 flex flex-col divide-y divide-slate-100">
                                <ReviewRow label="Lead guest" value=Signal::derive(move || {
                                    format!("{} {}", first_name.get().trim(), last_name.get().trim())
                                }) on_edit=move || step.set(0) />
                                <ReviewRow label="Email" value=Signal::derive(move || {
                                    let e = email.get();
                                    if e.trim().is_empty() { "—".into() } else { e }
                                }) on_edit=move || step.set(0) />
                                <ReviewRow label="Phone" value=Signal::derive(move || {
                                    let p = phone.get();
                                    if p.trim().is_empty() { "—".into() } else { p }
                                }) on_edit=move || step.set(0) />
                                <ReviewRow label="Stay" value=Signal::derive(move || format!(
                                    "{} → {}",
                                    pretty_date(Some(&check_in.get())),
                                    pretty_date(Some(&check_out.get())),
                                )) on_edit=move || step.set(1) />
                                <ReviewRow label="Guests" value=Signal::derive(move || pluralize(guests.get(), "guest"))
                                    on_edit=move || step.set(1) />
                                <ReviewRow label="Special requests" value=Signal::derive(move || {
                                    let r = requests.get();
                                    if r.trim().is_empty() { "None".into() } else { r }
                                }) on_edit=move || step.set(0) />
                            </dl>

                            <div class="mt-5 rounded-xl bg-blue-50 p-4 text-sm text-blue-900">
                                <p class="flex items-center gap-2 font-bold">
                                    <Icon name="info" class="h-4 w-4 shrink-0" />
                                    "What happens next"
                                </p>
                                <ul class="mt-2 flex list-inside list-disc flex-col gap-1">
                                    <li>"The hotel receives your request and confirms it."</li>
                                    <li>"Your booking reference appears on the next screen — keep it."</li>
                                    <li>"Pay the hotel directly on arrival. No card is taken here."</li>
                                </ul>
                            </div>
                        </section>
                    </Show>

                    // ---- Navigation --------------------------------------
                    <div class="mt-5 flex items-center justify-between gap-3">
                        <button
                            on:click=go_back
                            disabled=move || (step.get() == 0)
                            class="rounded-xl border border-slate-300 px-5 py-2.5 text-sm font-semibold text-slate-700 transition-colors hover:bg-slate-50 disabled:opacity-40"
                        >
                            "Back"
                        </button>

                        <Show
                            when=move || (step.get() < 2)
                            fallback=move || view! {
                                <button
                                    on:click=move |ev| submit.with_value(|f| f(ev))
                                    disabled=move || submitting.get()
                                    class="sheen flex items-center gap-2 rounded-xl bg-blue-700 px-6 py-3 text-sm font-bold text-white shadow-md shadow-blue-700/20 transition-all duration-200 hover:bg-blue-800 hover:shadow-lg active:scale-[0.98] disabled:opacity-60"
                                >
                                    {move || if submitting.get() { "Reserving…" } else { "Confirm reservation" }}
                                    <Icon name="check" class="h-4 w-4" />
                                </button>
                            }
                        >
                            <button
                                on:click=go_next
                                class="sheen flex items-center gap-2 rounded-xl bg-blue-700 px-6 py-3 text-sm font-bold text-white shadow-md shadow-blue-700/20 transition-all duration-200 hover:bg-blue-800 hover:shadow-lg active:scale-[0.98]"
                            >
                                "Continue"
                                <Icon name="arrow-right" class="h-4 w-4" />
                            </button>
                        </Show>
                    </div>
                </div>

                // ================= SUMMARY =================
                <aside class="lg:sticky lg:top-24 lg:self-start">
                    <div class="overflow-hidden rounded-2xl border border-slate-200 bg-white">
                        <img src=room_image alt=room_name.clone() class="h-32 w-full object-cover" />

                        <div class="p-4">
                            <p class="text-xs uppercase tracking-wide text-slate-400">{hotel_name.clone()}</p>
                            <h3 class="mt-0.5 font-bold text-ink">{room_name.clone()}</h3>
                            <p class="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-slate-500">
                                <span class="flex items-center gap-1">
                                    <Icon name="home" class="h-3.5 w-3.5" />
                                    {format!("Room {room_number}")}
                                </span>
                                <Show when={ let b = bed_summary.clone(); move || !b.is_empty() }>
                                    <span class="flex items-center gap-1">
                                        <Icon name="bed" class="h-3.5 w-3.5" />
                                        {bed_summary.clone()}
                                    </span>
                                </Show>
                            </p>
                            <Show when=move || breakfast>
                                <p class="mt-1.5 flex items-center gap-1 text-xs font-semibold text-emerald-700">
                                    <Icon name="coffee" class="h-3.5 w-3.5" />
                                    "Breakfast included"
                                </p>
                            </Show>

                            // ---- The stay ---------------------------------
                            // Mirrors the date fields so the chosen range is
                            // visible from every step, not just the one that
                            // sets it.
                            <div class="mt-3 rounded-xl bg-slate-50 px-3 py-2.5">
                                <div class="flex items-center justify-between gap-2">
                                    <span class="text-[11px] font-bold uppercase tracking-wider text-slate-400">
                                        "Your stay"
                                    </span>
                                    <span class="text-[11px] font-semibold text-slate-500">
                                        {move || {
                                            let n = nights.get();
                                            if n == 0 { "—".to_string() } else { pluralize(n, "night") }
                                        }}
                                    </span>
                                </div>
                                <div class="mt-1.5 flex items-center gap-2 text-xs text-slate-600">
                                    <Icon name="calendar" class="h-3.5 w-3.5 shrink-0 text-slate-400" />
                                    <span class="font-semibold text-ink">
                                        {move || pretty_date(Some(&check_in.get()))}
                                    </span>
                                    <Icon name="arrow-right" class="h-3 w-3 shrink-0 text-slate-300" />
                                    <span class="font-semibold text-ink">
                                        {move || pretty_date(Some(&check_out.get()))}
                                    </span>
                                </div>
                                <p class="mt-1 text-[11px] text-slate-400">
                                    {move || pluralize(guests.get(), "guest")}
                                </p>
                            </div>

                            // ---- Live price breakdown ---------------------
                            <div class="mt-4 border-t border-slate-100 pt-4">
                                <Suspense fallback=|| view! {
                                    <div class="skeleton h-24 rounded-xl"></div>
                                }>
                                    {move || Suspend::new(async move {
                                        match quote.await {
                                            Ok(Some(q)) => view! { <QuoteBreakdown quote=q currency=cur.get_value() /> }.into_any(),
                                            Ok(None) => view! {
                                                <p class="text-sm text-slate-500">
                                                    {format!(
                                                        "Published rate {} {} per night. Pick your dates to see the total.",
                                                        cur.get_value(),
                                                        money_round(nightly),
                                                    )}
                                                </p>
                                            }.into_any(),
                                            Err(e) => view! {
                                                <div>
                                                    <p class="rounded-xl bg-amber-50 px-3 py-2 text-xs text-amber-800">
                                                        {e.to_string()}
                                                    </p>
                                                    <p class="mt-2 text-xs text-slate-400">
                                                        {format!(
                                                            "Published rate {} {} per night.",
                                                            cur.get_value(),
                                                            money_round(nightly),
                                                        )}
                                                    </p>
                                                </div>
                                            }.into_any(),
                                        }
                                    })}
                                </Suspense>
                            </div>

                            <div class="mt-4 flex items-start gap-2 rounded-xl bg-emerald-50 p-3 text-xs text-emerald-900">
                                <Icon name="shield-check" class="mt-0.5 h-3.5 w-3.5 shrink-0" />
                                <span>
                                    <span class="font-bold">"No payment now. "</span>
                                    "You settle directly with the hotel on arrival, and this portal charges no booking fee."
                                </span>
                            </div>
                        </div>
                    </div>
                </aside>
            </div>
        </div>
    }
}

/// The API's own price breakdown, shown line for line.
#[component]
fn QuoteBreakdown(quote: Quote, currency: String) -> impl IntoView {
    let c = currency.clone();
    let line = move |label: String, amount: f64| {
        let c = c.clone();
        view! {
            <div class="flex items-baseline justify-between gap-3 text-sm">
                <dt class="text-slate-500">{label}</dt>
                <dd class="tabular-nums text-slate-700">{format!("{c} {}", money(amount))}</dd>
            </div>
        }
    };

    view! {
        <dl class="flex flex-col gap-1.5">
            {line(
                format!(
                    "{} x {}",
                    money_round(quote.daily_room_rate),
                    pluralize(quote.number_of_nights, "night"),
                ),
                quote.base_room_charge,
            )}
            {(quote.extra_bed_charge > 0.0).then(|| line(
                format!("Extra beds ({})", quote.extra_bed_count),
                quote.extra_bed_charge,
            ))}
            {(quote.pet_charge > 0.0).then(|| line("Pet charge".to_string(), quote.pet_charge))}
            {(quote.discount_amount > 0.0).then(|| line("Discount".to_string(), -quote.discount_amount))}
            {(quote.tax_amount > 0.0).then(|| line(
                format!("Tax ({:.0}%)", quote.tax_rate_percent),
                quote.tax_amount,
            ))}

            <div class="mt-2 flex items-baseline justify-between gap-3 border-t border-slate-100 pt-2">
                <dt class="text-sm font-bold text-ink">"Total"</dt>
                <dd class="text-lg font-bold tabular-nums text-ink">
                    {format!("{currency} {}", money(quote.total_amount))}
                </dd>
            </div>
            <p class="text-xs text-slate-400">"Payable at the hotel"</p>
        </dl>
    }
}

#[component]
fn Field(
    label: &'static str,
    value: RwSignal<String>,
    class: &'static str,
    #[prop(default = "text")] kind: &'static str,
    #[prop(default = "")] placeholder: &'static str,
) -> impl IntoView {
    view! {
        <div>
            <label class="mb-1.5 block text-sm font-semibold text-slate-700">{label}</label>
            <input type=kind placeholder=placeholder class=class
                prop:value=value on:input:target=move |ev| value.set(ev.target().value()) />
        </div>
    }
}

#[component]
fn Check(label: &'static str, hint: &'static str, value: RwSignal<bool>) -> impl IntoView {
    view! {
        <label class="flex cursor-pointer items-start gap-3 rounded-xl border border-slate-200 p-3 transition-colors hover:bg-slate-50">
            <input type="checkbox" class="mt-0.5 h-4 w-4 shrink-0" prop:checked=value
                on:change:target=move |ev| value.set(ev.target().checked()) />
            <span class="min-w-0">
                <span class="block text-sm font-medium text-slate-800">{label}</span>
                <span class="block text-xs text-slate-500">{hint}</span>
            </span>
        </label>
    }
}

#[component]
fn NumberField(value: RwSignal<u32>, min: u32, max: u32, icon: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2 rounded-xl border border-slate-300 px-3 py-2">
            <Icon name=icon class="h-4 w-4 shrink-0 text-slate-400" />
            <button
                type="button"
                class="flex h-7 w-7 items-center justify-center rounded-lg text-slate-500 transition-colors hover:bg-slate-100 disabled:opacity-30"
                disabled=move || (value.get() <= min)
                on:click=move |_| value.update(|v| *v = v.saturating_sub(1).max(min))
            >
                <Icon name="minus" class="h-3.5 w-3.5" />
            </button>
            <span class="min-w-6 flex-1 text-center text-sm font-bold tabular-nums text-slate-800">
                {move || value.get()}
            </span>
            <button
                type="button"
                class="flex h-7 w-7 items-center justify-center rounded-lg text-slate-500 transition-colors hover:bg-slate-100 disabled:opacity-30"
                disabled=move || (value.get() >= max)
                on:click=move |_| value.update(|v| *v = (*v + 1).min(max))
            >
                <Icon name="plus" class="h-3.5 w-3.5" />
            </button>
        </div>
    }
}

#[component]
fn ReviewRow(
    label: &'static str,
    value: Signal<String>,
    on_edit: impl Fn() + Copy + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <div class="flex items-start justify-between gap-3 py-2.5">
            <dt class="shrink-0 text-sm text-slate-500">{label}</dt>
            <dd class="flex min-w-0 items-start gap-2">
                <span class="min-w-0 break-words text-right text-sm font-semibold text-slate-800">
                    {move || value.get()}
                </span>
                <button
                    class="shrink-0 text-xs font-semibold text-blue-700 hover:underline"
                    on:click=move |_| on_edit()
                >
                    "Edit"
                </button>
            </dd>
        </div>
    }
}
