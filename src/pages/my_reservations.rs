use crate::components::{pluralize, thousands, use_toast, EmptyState, Icon, Modal};
use crate::data::{find_hotel, Hotel};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Upcoming,
    Past,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
struct Booking {
    reference: &'static str,
    hotel_id: &'static str,
    room: &'static str,
    check_in: &'static str,
    check_out: &'static str,
    nights: u32,
    guests: &'static str,
    total: u32,
    status: Tab,
    /// Days until check-in, shown as a countdown on upcoming stays.
    days_away: i32,
}

const BOOKINGS: &[Booking] = &[
    Booking { reference: "HA-250143", hotel_id: "golden-tulip-addis-ababa", room: "Deluxe Room", check_in: "4 Sep 2026", check_out: "6 Sep 2026", nights: 2, guests: "2 guests · 1 room", total: 8280, status: Tab::Upcoming, days_away: 9 },
    Booking { reference: "HA-250189", hotel_id: "haile-grand-addis-hotel", room: "Twin Room", check_in: "21 Sep 2026", check_out: "22 Sep 2026", nights: 1, guests: "1 guest · 1 room", total: 2415, status: Tab::Upcoming, days_away: 26 },
    Booking { reference: "HA-250067", hotel_id: "sarova-stanley-nairobi", room: "Deluxe King", check_in: "12 Jul 2026", check_out: "15 Jul 2026", nights: 3, guests: "2 guests · 1 room", total: 17940, status: Tab::Past, days_away: -45 },
    Booking { reference: "HA-249981", hotel_id: "ambassador-hotel-hargeisa", room: "Standard Room", check_in: "2 Jun 2026", check_out: "4 Jun 2026", nights: 2, guests: "2 guests · 1 room", total: 6440, status: Tab::Past, days_away: -85 },
    Booking { reference: "HA-250112", hotel_id: "jazeera-palace-mogadishu", room: "Executive Suite", check_in: "18 Aug 2026", check_out: "20 Aug 2026", nights: 2, guests: "1 guest · 1 room", total: 11040, status: Tab::Cancelled, days_away: -8 },
];

#[component]
pub fn MyReservationsPage() -> impl IntoView {
    let toast = use_toast();
    let tab = RwSignal::new(Tab::Upcoming);
    let cancelling = RwSignal::new(Option::<&'static str>::None);
    let cancelled = RwSignal::new(Vec::<&'static str>::new());
    let menu_open = RwSignal::new(Option::<&'static str>::None);

    let effective_status = move |b: &Booking| {
        if cancelled.get().contains(&b.reference) { Tab::Cancelled } else { b.status }
    };

    let count = move |t: Tab| BOOKINGS.iter().filter(|b| effective_status(b) == t).count();

    let visible = move || {
        BOOKINGS
            .iter()
            .filter(|b| effective_status(b) == tab.get())
            .collect::<Vec<_>>()
    };

    let confirm_cancel = move |_| {
        if let Some(reference) = cancelling.get() {
            cancelled.update(|list| list.push(reference));
            toast.success(
                "Reservation cancelled",
                format!("{reference} has been cancelled and the hotel notified. No charge applies."),
            );
            cancelling.set(None);
            tab.set(Tab::Cancelled);
        }
    };

    view! {
        <Title text="My Reservation — Horn of Africa Hotel Portal" />

        <div class="mx-auto max-w-3xl px-4 py-7 pb-24 sm:pb-7">
            <div class="animate-fade-up">
                <h1 class="text-2xl font-extrabold tracking-tight text-slate-900">"My reservations"</h1>
                <p class="mt-1 text-sm text-slate-500">
                    "Everything you have booked from this device, plus anything you have retrieved with a reference."
                </p>
            </div>

            <div class="mt-4 flex animate-fade-up items-start gap-2.5 rounded-xl bg-blue-50 p-3.5 text-sm text-blue-900 ring-1 ring-blue-100" style="animation-delay: 60ms">
                <Icon name="info" class="mt-0.5 h-4 w-4 shrink-0" />
                <span>
                    "Reservations are stored on this device. Using a different phone or browser? "
                    <A href="/retrieve-booking" attr:class="font-bold underline">"Retrieve them with your reference"</A>
                    "."
                </span>
            </div>

            // ---- Tabs -------------------------------------------------------
            <div class="mt-5 flex animate-fade-up gap-1 rounded-xl bg-slate-100 p-1" style="animation-delay: 100ms">
                {[(Tab::Upcoming, "Upcoming"), (Tab::Past, "Past"), (Tab::Cancelled, "Cancelled")]
                    .into_iter()
                    .map(|(t, label)| view! {
                        <button
                            on:click=move |_| tab.set(t)
                            class=move || format!(
                                "flex flex-1 items-center justify-center gap-1.5 rounded-lg px-3 py-2.5 text-sm font-semibold transition-all duration-200 {}",
                                if tab.get() == t { "bg-white text-blue-700 shadow-sm" } else { "text-slate-500 hover:text-slate-700" }
                            )
                        >
                            {label}
                            <span class=move || format!(
                                "flex h-5 min-w-5 items-center justify-center rounded-full px-1.5 text-[11px] font-bold transition-colors {}",
                                if tab.get() == t { "bg-blue-100 text-blue-700" } else { "bg-slate-200 text-slate-500" }
                            )>
                                {move || count(t)}
                            </span>
                        </button>
                    })
                    .collect_view()}
            </div>

            // ---- List -------------------------------------------------------
            <div class="mt-5">
                <Show
                    when=move || !visible().is_empty()
                    fallback=move || {
                        let (title, body) = match tab.get() {
                            Tab::Upcoming => ("No upcoming stays", "When you reserve a room it will appear here with everything you need for check-in."),
                            Tab::Past => ("No past stays yet", "Completed reservations are kept here so you can rebook a hotel you liked."),
                            Tab::Cancelled => ("Nothing cancelled", "Cancelled reservations stay here for your records."),
                        };
                        view! {
                            <EmptyState icon="ticket" title=title body=body>
                                <A
                                    href="/hotels"
                                    attr:class="rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800"
                                >
                                    "Find a hotel"
                                </A>
                            </EmptyState>
                        }
                    }
                >
                    <div class="flex flex-col gap-4">
                        {move || visible().into_iter().enumerate().map(|(i, b)| {
                            let delay = format!("animation-delay: {}ms", i * 70);
                            let hotel: &'static Hotel = find_hotel(b.hotel_id).unwrap_or(&crate::data::HOTELS[0]);
                            let status = effective_status(b);
                            let reference = b.reference;
                            view! {
                                <article class="animate-fade-up overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm transition-shadow duration-300 hover:shadow-md" style=delay>
                                    <Show when=move || (status == Tab::Upcoming && b.days_away <= 14)>
                                        <div class="flex items-center gap-2 bg-gradient-to-r from-blue-700 to-indigo-700 px-4 py-2 text-xs font-bold text-white">
                                            <Icon name="clock" class="h-3.5 w-3.5" />
                                            {format!("Check-in in {} days", b.days_away)}
                                        </div>
                                    </Show>

                                    <div class="p-4">
                                        <div class="flex gap-3.5">
                                            <img src=hotel.image alt=hotel.name class="h-20 w-20 shrink-0 rounded-xl object-cover" />
                                            <div class="min-w-0 flex-1">
                                                <div class="flex items-start justify-between gap-2">
                                                    <div class="min-w-0">
                                                        <p class="truncate font-bold text-slate-900">{hotel.name}</p>
                                                        <p class="flex items-center gap-1 truncate text-xs text-slate-500">
                                                            <Icon name="map-pin" class="h-3 w-3 shrink-0" />
                                                            {hotel.location_line()}
                                                        </p>
                                                        <p class="mt-0.5 truncate text-xs text-slate-500">{b.room}</p>
                                                    </div>

                                                    <div class="flex shrink-0 items-center gap-1.5">
                                                        <span class=format!(
                                                            "rounded-full px-2.5 py-1 text-[11px] font-bold {}",
                                                            match status {
                                                                Tab::Upcoming => "bg-emerald-100 text-emerald-700",
                                                                Tab::Past => "bg-slate-100 text-slate-600",
                                                                Tab::Cancelled => "bg-red-100 text-red-700",
                                                            }
                                                        )>
                                                            {match status {
                                                                Tab::Upcoming => "Upcoming",
                                                                Tab::Past => "Completed",
                                                                Tab::Cancelled => "Cancelled",
                                                            }}
                                                        </span>
                                                        <div class="relative">
                                                            <button
                                                                aria-label="More actions"
                                                                on:click=move |_| menu_open.update(|m| {
                                                                    *m = if *m == Some(reference) { None } else { Some(reference) };
                                                                })
                                                                class="flex h-7 w-7 items-center justify-center rounded-lg text-slate-400 transition-colors hover:bg-slate-100 hover:text-slate-700"
                                                            >
                                                                <Icon name="more" class="h-4 w-4" />
                                                            </button>
                                                            <Show when=move || (menu_open.get() == Some(reference))>
                                                                <div class="absolute right-0 top-full z-20 mt-1 w-48 animate-fade-down overflow-hidden rounded-xl border border-slate-200 bg-white p-1 shadow-xl">
                                                                    <A
                                                                        href=format!("/reservation/{reference}")
                                                                        attr:class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-slate-600 transition-colors hover:bg-slate-50"
                                                                    >
                                                                        <Icon name="eye" class="h-3.5 w-3.5" />
                                                                        "View details"
                                                                    </A>
                                                                    <a
                                                                        href=format!("tel:{}", hotel.phone.replace(' ', ""))
                                                                        class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-slate-600 transition-colors hover:bg-slate-50"
                                                                    >
                                                                        <Icon name="phone" class="h-3.5 w-3.5" />
                                                                        "Call the hotel"
                                                                    </a>
                                                                    <A
                                                                        href=format!("/hotels/{}", hotel.id)
                                                                        attr:class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-slate-600 transition-colors hover:bg-slate-50"
                                                                    >
                                                                        <Icon name="building" class="h-3.5 w-3.5" />
                                                                        "Book again"
                                                                    </A>
                                                                    <Show when=move || (status == Tab::Upcoming)>
                                                                        <button
                                                                            on:click=move |_| { menu_open.set(None); cancelling.set(Some(reference)); }
                                                                            class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm text-red-600 transition-colors hover:bg-red-50"
                                                                        >
                                                                            <Icon name="x-circle" class="h-3.5 w-3.5" />
                                                                            "Cancel reservation"
                                                                        </button>
                                                                    </Show>
                                                                </div>
                                                            </Show>
                                                        </div>
                                                    </div>
                                                </div>

                                                <dl class="mt-3 grid grid-cols-3 gap-2 text-xs">
                                                    <div>
                                                        <dt class="text-slate-400">"Check-in"</dt>
                                                        <dd class="font-semibold text-slate-700">{b.check_in}</dd>
                                                    </div>
                                                    <div>
                                                        <dt class="text-slate-400">"Check-out"</dt>
                                                        <dd class="font-semibold text-slate-700">{b.check_out}</dd>
                                                    </div>
                                                    <div>
                                                        <dt class="text-slate-400">"Guests"</dt>
                                                        <dd class="font-semibold text-slate-700">{b.guests}</dd>
                                                    </div>
                                                </dl>
                                            </div>
                                        </div>

                                        <div class="mt-3.5 flex items-center justify-between border-t border-slate-100 pt-3.5">
                                            <A
                                                href=format!("/reservation/{reference}")
                                                attr:class="flex items-center gap-1.5 font-mono text-sm font-bold text-blue-700 hover:underline"
                                            >
                                                <Icon name="ticket" class="h-3.5 w-3.5" />
                                                {reference}
                                            </A>
                                            <span class="text-right">
                                                <span class="block text-[11px] text-slate-400">{pluralize(b.nights, "night")}</span>
                                                <span class="block font-extrabold text-slate-900 tabular-nums">
                                                    {format!("ETB {}", thousands(b.total))}
                                                </span>
                                            </span>
                                        </div>

                                        <div class="mt-3 grid grid-cols-3 gap-2 text-xs">
                                            <A
                                                href=format!("/reservation/{reference}")
                                                attr:class="flex items-center justify-center gap-1.5 rounded-lg border border-slate-300 py-2.5 font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700"
                                            >
                                                <Icon name="eye" class="h-3.5 w-3.5" />
                                                "Details"
                                            </A>
                                            <a
                                                href=format!(
                                                    "https://wa.me/{}?text=Booking%20{}",
                                                    hotel.whatsapp.replace([' ', '+'], ""),
                                                    reference
                                                )
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="flex items-center justify-center gap-1.5 rounded-lg border border-slate-300 py-2.5 font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-emerald-300 hover:text-emerald-700"
                                            >
                                                <Icon name="message" class="h-3.5 w-3.5" />
                                                "Message"
                                            </a>
                                            <a
                                                href=format!("tel:{}", hotel.phone.replace(' ', ""))
                                                class="flex items-center justify-center gap-1.5 rounded-lg border border-slate-300 py-2.5 font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700"
                                            >
                                                <Icon name="phone" class="h-3.5 w-3.5" />
                                                "Call"
                                            </a>
                                        </div>
                                    </div>
                                </article>
                            }
                        }).collect_view()}
                    </div>
                </Show>
            </div>

            <A
                href="/retrieve-booking"
                attr:class="mt-6 flex w-full items-center justify-center gap-2 rounded-xl border-2 border-dashed border-slate-300 py-4 text-sm font-bold text-slate-600 transition-all duration-200 hover:border-blue-400 hover:bg-blue-50/40 hover:text-blue-700"
            >
                <Icon name="search" class="h-4 w-4" />
                "Retrieve another booking"
            </A>
        </div>

        // ---- Cancel confirmation ----------------------------------------
        <Show when=move || cancelling.get().is_some()>
            <Modal
                title="Cancel this reservation?"
                width="max-w-md"
                on_close=move || cancelling.set(None)
            >
                <div class="flex items-start gap-3 rounded-xl bg-amber-50 p-4 text-sm text-amber-900 ring-1 ring-amber-100">
                    <Icon name="alert" class="mt-0.5 h-4.5 w-4.5 shrink-0" />
                    <span>
                        "The hotel will be notified immediately and the room released. This cannot be undone — you would need to make a new reservation."
                    </span>
                </div>

                <p class="mt-4 text-sm text-slate-600">
                    "Reference "
                    <span class="font-mono font-bold text-slate-900">{move || cancelling.get().unwrap_or("")}</span>
                    " is within its free cancellation window, so nothing will be charged."
                </p>

                <div class="mt-6 flex gap-2.5">
                    <button
                        on:click=move |_| cancelling.set(None)
                        class="flex-1 rounded-xl border border-slate-300 py-3 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50"
                    >
                        "Keep reservation"
                    </button>
                    <button
                        on:click=confirm_cancel
                        class="flex-1 rounded-xl bg-red-600 py-3 text-sm font-bold text-white shadow-lg shadow-red-600/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-red-700 active:scale-[0.98]"
                    >
                        "Yes, cancel it"
                    </button>
                </div>
            </Modal>
        </Show>
    }
}
