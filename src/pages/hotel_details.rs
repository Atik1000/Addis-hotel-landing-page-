use crate::components::{pluralize, tag_icon, thousands, AccordionItem, Breadcrumbs, Gallery, HotelCardCompact, Icon, RatingBadge, Stars};
use crate::data::{find_hotel, similar_hotels, Hotel};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

/// Groups amenities so the full list reads as categories rather than a wall of chips.
fn amenity_group(a: &str) -> &'static str {
    match a {
        "Free Wi-Fi" | "Business Centre" | "Air Conditioning" => "In every room",
        "Swimming Pool" | "Gym" | "Spa" => "Wellness & leisure",
        "Restaurant" | "Bar" | "Room Service" => "Food & drink",
        "Airport Pickup" | "Parking" => "Getting around",
        _ => "Services",
    }
}

const GROUP_ORDER: [&str; 5] = [
    "In every room",
    "Food & drink",
    "Wellness & leisure",
    "Getting around",
    "Services",
];

#[component]
pub fn HotelDetailsPage() -> impl IntoView {
    let params = use_params_map();
    let hotel = move || {
        let id = params.get().get("id").unwrap_or_default();
        find_hotel(&id)
    };

    view! {
        {move || match hotel() {
            None => view! {
                <div class="mx-auto flex max-w-lg flex-col items-center gap-3 px-4 py-24 text-center">
                    <span class="flex h-16 w-16 items-center justify-center rounded-full bg-slate-100 text-slate-400">
                        <Icon name="building" class="h-7 w-7" />
                    </span>
                    <h1 class="text-xl font-bold text-slate-900">"Hotel not found"</h1>
                    <p class="text-sm text-slate-500">"That listing may have been removed or the link is incorrect."</p>
                    <A href="/hotels" attr:class="mt-2 rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800">
                        "Browse all hotels"
                    </A>
                </div>
            }.into_any(),
            Some(h) => view! { <HotelDetail hotel=h /> }.into_any(),
        }}
    }
}

#[component]
fn HotelDetail(hotel: &'static Hotel) -> impl IntoView {
    let h = hotel;
    let description_expanded = RwSignal::new(false);
    let show_all_amenities = RwSignal::new(false);
    let reviews_shown = RwSignal::new(3usize);
    let breakdown = h.rating_breakdown();
    let total_reviews: u32 = breakdown.iter().sum::<u32>().max(1);
    let cheapest = h.cheapest_room();
    let map_url = format!(
        "https://www.google.com/maps/search/?api=1&query={}",
        h.location_line().replace(' ', "+")
    );

    let trail = vec![
        ("Home".to_string(), Some("/".to_string())),
        (h.country.to_string(), Some("/hotels".to_string())),
        (h.city.to_string(), Some(format!("/hotels?city={}", h.city.replace(' ', "+")))),
        (h.name.to_string(), None),
    ];

    view! {
        <Title text=format!("{} — {}, {} | Horn of Africa Hotel Portal", h.name, h.area, h.city) />

        <div class="mx-auto max-w-6xl px-4 py-5">
            <div class="mb-4 animate-fade-up">
                <Breadcrumbs trail=trail />
            </div>

            <div class="animate-fade-up" style="animation-delay: 60ms">
                <Gallery photos=h.photos alt=h.name />
            </div>

            <div class="mt-7 grid gap-8 lg:grid-cols-[minmax(0,1fr)_21rem]">
                // ================= MAIN COLUMN =================
                <div class="min-w-0">
                    <div class="animate-fade-up" style="animation-delay: 100ms">
                        <div class="flex flex-wrap items-center gap-2">
                            <span class="flex items-center gap-0.5 text-amber-500">
                                {(0..h.star_class).map(|_| view! { <Icon name="star" class="h-3.5 w-3.5" /> }).collect_view()}
                            </span>
                            <span class="text-[11px] font-bold uppercase tracking-wider text-slate-400">
                                {format!("{}-star hotel", h.star_class)}
                            </span>
                            <Show when=move || h.featured>
                                <span class="flex items-center gap-1 rounded-full bg-blue-50 px-2.5 py-0.5 text-[11px] font-bold text-blue-700 ring-1 ring-blue-100">
                                    <Icon name="award" class="h-3 w-3" />
                                    "Featured"
                                </span>
                            </Show>
                            <span class="flex items-center gap-1 rounded-full bg-emerald-50 px-2.5 py-0.5 text-[11px] font-bold text-emerald-700 ring-1 ring-emerald-100">
                                <Icon name="shield-check" class="h-3 w-3" />
                                "Verified property"
                            </span>
                        </div>

                        <h1 class="mt-2 text-3xl font-extrabold tracking-tight text-slate-900">{h.name}</h1>

                        <p class="mt-1.5 flex items-center gap-1.5 text-sm text-slate-500">
                            <Icon name="map-pin" class="h-4 w-4 shrink-0 text-blue-700" />
                            {h.location_line()}
                            <a href=map_url.clone() target="_blank" rel="noopener noreferrer" class="ml-1 font-semibold text-blue-700 hover:underline">
                                "Show on map"
                            </a>
                        </p>

                        <div class="mt-3">
                            <RatingBadge rating=h.rating review_count=h.review_count />
                        </div>
                    </div>

                    // ---- Quick facts ------------------------------------------
                    <div class="mt-6 grid animate-fade-up grid-cols-2 gap-3 sm:grid-cols-4" style="animation-delay: 140ms">
                        {[
                            ("clock", "Check-in", h.check_in_time),
                            ("clock", "Check-out", h.check_out_time),
                            ("bed", "Room types", "See below"),
                            ("wallet", "Payment", "At the hotel"),
                        ].into_iter().enumerate().map(|(i, (icon, label, value))| {
                            let value = if label == "Room types" {
                                Box::leak(format!("{} available", h.rooms.len()).into_boxed_str()) as &'static str
                            } else { value };
                            let delay = format!("animation-delay: {}ms", 140 + i * 40);
                            view! {
                                <div class="flex items-center gap-2.5 rounded-xl border border-slate-200 bg-white p-3" style=delay>
                                    <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-slate-100 text-slate-600">
                                        <Icon name=icon class="h-4 w-4" />
                                    </span>
                                    <span class="min-w-0">
                                        <span class="block text-[11px] uppercase tracking-wide text-slate-400">{label}</span>
                                        <span class="block truncate text-sm font-bold text-slate-800">{value}</span>
                                    </span>
                                </div>
                            }
                        }).collect_view()}
                    </div>

                    // ---- About ------------------------------------------------
                    <section class="mt-8">
                        <h2 class="text-lg font-bold text-slate-900">"About this hotel"</h2>
                        <p class=move || format!(
                            "mt-2 text-sm leading-relaxed text-slate-600 transition-all {}",
                            if description_expanded.get() { "" } else { "line-clamp-3" }
                        )>
                            {h.description}
                        </p>
                        <button
                            on:click=move |_| description_expanded.update(|v| *v = !*v)
                            class="mt-2 flex items-center gap-1 text-sm font-bold text-blue-700 hover:underline"
                        >
                            {move || if description_expanded.get() { "Read less" } else { "Read more" }}
                            <span class=move || format!(
                                "transition-transform duration-300 {}",
                                if description_expanded.get() { "rotate-180" } else { "" }
                            )>
                                <Icon name="chevron-down" class="h-3.5 w-3.5" />
                            </span>
                        </button>
                    </section>

                    // ---- Amenities --------------------------------------------
                    <section class="mt-8">
                        <div class="flex items-center justify-between gap-3">
                            <h2 class="text-lg font-bold text-slate-900">"Amenities"</h2>
                            <button
                                on:click=move |_| show_all_amenities.update(|v| *v = !*v)
                                class="text-sm font-bold text-blue-700 hover:underline"
                            >
                                {move || if show_all_amenities.get() { "Show less".to_string() } else { format!("Show all {}", h.amenities.len()) }}
                            </button>
                        </div>

                        <Show
                            when=move || show_all_amenities.get()
                            fallback=move || view! {
                                <div class="mt-3 grid grid-cols-2 gap-2 sm:grid-cols-3">
                                    {h.amenities.iter().take(6).map(|a| view! {
                                        <span class="flex items-center gap-2 rounded-xl border border-slate-200 bg-white px-3 py-2.5 text-sm text-slate-700 transition-colors hover:border-blue-200 hover:bg-blue-50/50">
                                            <Icon name=tag_icon(a) class="h-4 w-4 shrink-0 text-blue-700" />
                                            <span class="truncate">{*a}</span>
                                        </span>
                                    }).collect_view()}
                                </div>
                            }
                        >
                            <div class="mt-3 flex animate-fade-up flex-col gap-5">
                                {GROUP_ORDER.into_iter().map(|group| {
                                    let items: Vec<&&str> = h.amenities.iter().filter(|a| amenity_group(a) == group).collect();
                                    let empty = items.is_empty();
                                    view! {
                                        <Show when=move || !empty>
                                            <div>
                                                <h3 class="mb-2 text-xs font-bold uppercase tracking-wider text-slate-400">{group}</h3>
                                                <div class="grid grid-cols-2 gap-2 sm:grid-cols-3">
                                                    {h.amenities.iter().filter(|a| amenity_group(a) == group).map(|a| view! {
                                                        <span class="flex items-center gap-2 rounded-xl border border-slate-200 bg-white px-3 py-2.5 text-sm text-slate-700">
                                                            <Icon name=tag_icon(a) class="h-4 w-4 shrink-0 text-blue-700" />
                                                            <span class="truncate">{*a}</span>
                                                        </span>
                                                    }).collect_view()}
                                                </div>
                                            </div>
                                        </Show>
                                    }
                                }).collect_view()}
                            </div>
                        </Show>
                    </section>

                    // ---- Rooms -------------------------------------------------
                    <section class="mt-9" id="rooms">
                        <h2 class="text-lg font-bold text-slate-900">"Choose your room"</h2>
                        <p class="mt-1 text-sm text-slate-500">"All rates are per room per night and paid at the hotel."</p>

                        <div class="mt-4 flex flex-col gap-4">
                            {h.rooms.iter().enumerate().map(|(i, r)| {
                                let delay = format!("animation-delay: {}ms", i * 70);
                                view! {
                                    <article class="reveal card-hover flex flex-col overflow-hidden rounded-2xl border border-slate-200 bg-white sm:flex-row" style=delay>
                                        <div class="skeleton relative h-44 w-full shrink-0 overflow-hidden sm:h-auto sm:w-56">
                                            <img src=r.image alt=r.name loading="lazy" class="h-full w-full object-cover transition-transform duration-700 hover:scale-105" />
                                            <Show when=move || (r.rooms_left <= 3)>
                                                <span class="absolute bottom-2 left-2 flex items-center gap-1 rounded-full bg-red-600 px-2.5 py-1 text-[11px] font-bold text-white shadow-md">
                                                    <Icon name="alert" class="h-3 w-3" />
                                                    {format!("Only {} left", r.rooms_left)}
                                                </span>
                                            </Show>
                                        </div>

                                        <div class="flex flex-1 flex-col gap-4 p-4 sm:flex-row">
                                            <div class="min-w-0 flex-1">
                                                <h3 class="text-base font-bold text-slate-900">{r.name}</h3>
                                                <p class="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-slate-500">
                                                    <span class="flex items-center gap-1"><Icon name="bed" class="h-3.5 w-3.5" />{r.beds}</span>
                                                    <span class="flex items-center gap-1"><Icon name="users" class="h-3.5 w-3.5" />{pluralize(r.guests, "guest")}</span>
                                                    <span class="flex items-center gap-1"><Icon name="scan" class="h-3.5 w-3.5" />{format!("{} m²", r.size_sqm)}</span>
                                                </p>

                                                <ul class="mt-3 flex flex-col gap-1.5">
                                                    {r.perks.iter().map(|p| view! {
                                                        <li class="flex items-center gap-1.5 text-xs text-slate-600">
                                                            <Icon name="check" class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                                            {*p}
                                                        </li>
                                                    }).collect_view()}
                                                    <li class=format!(
                                                        "flex items-center gap-1.5 text-xs font-semibold {}",
                                                        if r.refundable { "text-emerald-700" } else { "text-slate-400" }
                                                    )>
                                                        <Icon
                                                            name=if r.refundable { "check-circle" } else { "info" }
                                                            class="h-3.5 w-3.5 shrink-0"
                                                        />
                                                        {if r.refundable { "Free cancellation up to 24h before" } else { "Non-refundable rate" }}
                                                    </li>
                                                </ul>
                                            </div>

                                            <div class="flex shrink-0 items-end justify-between gap-3 border-t border-slate-100 pt-3 sm:w-40 sm:flex-col sm:items-end sm:border-l sm:border-t-0 sm:pl-4 sm:pt-0">
                                                <div class="sm:text-right">
                                                    <p class="text-xl font-extrabold text-slate-900">{format!("ETB {}", thousands(r.price_per_night))}</p>
                                                    <p class="text-xs text-slate-400">"per night"</p>
                                                </div>
                                                <A
                                                    href=format!("/hotels/{}/reserve/{}", h.id, r.id)
                                                    attr:class="sheen flex items-center justify-center gap-1.5 rounded-xl bg-blue-700 px-4 py-2.5 text-sm font-bold text-white shadow-md shadow-blue-700/20 transition-all duration-200 hover:bg-blue-800 hover:shadow-lg active:scale-[0.98] sm:w-full"
                                                >
                                                    "Reserve"
                                                    <Icon name="arrow-right" class="h-3.5 w-3.5" />
                                                </A>
                                            </div>
                                        </div>
                                    </article>
                                }
                            }).collect_view()}
                        </div>
                    </section>

                    // ---- Reviews -----------------------------------------------
                    <section class="mt-10">
                        <h2 class="text-lg font-bold text-slate-900">"Guest reviews"</h2>

                        <div class="mt-4 grid gap-6 rounded-2xl border border-slate-200 bg-white p-5 sm:grid-cols-[auto_minmax(0,1fr)]">
                            <div class="flex flex-col items-center justify-center gap-1 border-slate-100 sm:border-r sm:pr-6">
                                <span class="text-4xl font-extrabold tracking-tight text-slate-900">{format!("{:.1}", h.rating)}</span>
                                <Stars rating=h.rating class="h-4 w-4" />
                                <span class="text-xs text-slate-400">{pluralize(h.review_count, "review")}</span>
                            </div>

                            <div class="flex flex-col gap-2">
                                {(0..5).rev().map(|i| {
                                    let stars = i + 1;
                                    let count = breakdown[i as usize];
                                    let pct = count * 100 / total_reviews;
                                    let width = format!("width: {pct}%");
                                    view! {
                                        <div class="flex items-center gap-3 text-xs">
                                            <span class="flex w-10 shrink-0 items-center gap-0.5 font-semibold text-slate-600">
                                                {stars}
                                                <Icon name="star" class="h-3 w-3 text-amber-500" />
                                            </span>
                                            <span class="h-2 flex-1 overflow-hidden rounded-full bg-slate-100">
                                                <span class="animate-width-grow block h-full rounded-full bg-amber-400" style=width></span>
                                            </span>
                                            <span class="w-10 shrink-0 text-right tabular-nums text-slate-400">{count}</span>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        <div class="mt-4 flex flex-col gap-3">
                            {move || h.reviews.iter().take(reviews_shown.get()).enumerate().map(|(i, r)| {
                                let delay = format!("animation-delay: {}ms", (i % 3) * 70);
                                view! {
                                    <article class="animate-fade-up rounded-2xl border border-slate-200 bg-white p-5" style=delay>
                                        <div class="flex items-start justify-between gap-3">
                                            <div class="flex items-center gap-3">
                                                <span class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-blue-100 text-sm font-bold text-blue-700">
                                                    {r.initials}
                                                </span>
                                                <span>
                                                    <span class="block text-sm font-bold text-slate-900">{r.author}</span>
                                                    <span class="block text-xs text-slate-400">{format!("{} · {}", r.country, r.date)}</span>
                                                </span>
                                            </div>
                                            <Stars rating={r.rating as f32} class="h-3.5 w-3.5" />
                                        </div>
                                        <h3 class="mt-3 text-sm font-bold text-slate-800">{r.title}</h3>
                                        <p class="mt-1 text-sm leading-relaxed text-slate-600">{r.body}</p>
                                        <p class="mt-2.5 flex items-center gap-1.5 text-xs text-slate-400">
                                            <Icon name="bed" class="h-3.5 w-3.5" />
                                            {r.stayed_in}
                                        </p>
                                    </article>
                                }
                            }).collect_view()}
                        </div>

                        <Show when=move || (reviews_shown.get() < h.reviews.len())>
                            <button
                                on:click=move |_| reviews_shown.update(|n| *n = h.reviews.len())
                                class="mt-4 flex w-full items-center justify-center gap-1.5 rounded-xl border border-slate-300 py-3 text-sm font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                            >
                                {move || format!("Show all {}", pluralize(h.reviews.len() as u32, "review"))}
                                <Icon name="chevron-down" class="h-4 w-4" />
                            </button>
                        </Show>
                    </section>

                    // ---- Location ----------------------------------------------
                    <section class="mt-10">
                        <h2 class="text-lg font-bold text-slate-900">"What's nearby"</h2>
                        <div class="mt-3 grid gap-3 sm:grid-cols-2">
                            {h.landmarks.iter().enumerate().map(|(i, l)| {
                                let delay = format!("animation-delay: {}ms", i * 50);
                                view! {
                                    <div class="reveal flex items-center justify-between gap-3 rounded-xl border border-slate-200 bg-white px-4 py-3" style=delay>
                                        <span class="flex min-w-0 items-center gap-2.5">
                                            <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-700">
                                                <Icon name=l.icon class="h-4 w-4" />
                                            </span>
                                            <span class="truncate text-sm text-slate-700">{l.name}</span>
                                        </span>
                                        <span class="shrink-0 text-sm font-bold text-slate-500">{l.distance}</span>
                                    </div>
                                }
                            }).collect_view()}
                        </div>

                        <a
                            href=map_url.clone()
                            target="_blank"
                            rel="noopener noreferrer"
                            class="group mt-3 flex items-center justify-between gap-3 rounded-2xl border border-blue-200 bg-gradient-to-r from-blue-50 to-indigo-50 px-5 py-4 transition-all duration-200 hover:-translate-y-0.5 hover:shadow-md"
                        >
                            <span class="flex items-center gap-3">
                                <span class="flex h-10 w-10 items-center justify-center rounded-xl bg-blue-700 text-white">
                                    <Icon name="map" class="h-5 w-5" />
                                </span>
                                <span>
                                    <span class="block text-sm font-bold text-slate-800">"Open in Google Maps"</span>
                                    <span class="block text-xs text-slate-500">{h.location_line()}</span>
                                </span>
                            </span>
                            <Icon name="external-link" class="h-4 w-4 text-blue-700 transition-transform duration-200 group-hover:translate-x-0.5" />
                        </a>
                    </section>

                    // ---- Policies -----------------------------------------------
                    <section class="mt-10">
                        <h2 class="text-lg font-bold text-slate-900">"Hotel policies"</h2>
                        <div class="mt-3 flex flex-col gap-3">
                            <AccordionItem
                                start_open=true
                                question="Check-in & check-out"
                                answer=Box::leak(format!(
                                    "Check-in from {}. Check-out by {}. The front desk is staffed 24 hours, so late arrivals are fine — call the hotel on the number in your confirmation if you expect to arrive after midnight.",
                                    h.check_in_time, h.check_out_time
                                ).into_boxed_str())
                            />
                            <AccordionItem
                                question="Payment"
                                answer="Payment is made directly to the hotel at check-in. No card details are collected online and no deposit is taken by the portal. The hotel accepts cash and major cards; mobile money availability varies by property."
                            />
                            <AccordionItem
                                question="Cancellation"
                                answer="Rooms marked 'Free cancellation' can be cancelled up to 24 hours before check-in at no cost. Non-refundable rates are charged in full if cancelled or if you do not arrive. You can cancel from My Reservation or by calling the hotel."
                            />
                            <AccordionItem
                                question="Children & extra beds"
                                answer="Children under 6 stay free when sharing an existing bed. Extra beds and cots are subject to availability and may carry a small charge — request them in the notes field when you reserve."
                            />
                            <AccordionItem
                                question="Identification"
                                answer="All guests must present a valid passport or national ID at check-in. For guests booking on someone else's behalf, the named guest must be present with their own identification."
                            />
                        </div>
                    </section>

                    // ---- Similar -------------------------------------------------
                    <section class="mt-10">
                        <h2 class="text-lg font-bold text-slate-900">{format!("More hotels in {}", h.city)}</h2>
                        <div class="mt-4 grid gap-4 sm:grid-cols-3">
                            {similar_hotels(h).into_iter().enumerate().map(|(i, s)| {
                                let delay = format!("animation-delay: {}ms", i * 70);
                                view! { <div class="reveal" style=delay><HotelCardCompact hotel=s /></div> }
                            }).collect_view()}
                        </div>
                    </section>
                </div>

                // ================= STICKY BOOKING RAIL =================
                <aside class="lg:sticky lg:top-24 lg:h-fit">
                    <div class="animate-slide-in-right overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-lg shadow-slate-900/5" style="animation-delay: 160ms">
                        <div class="border-b border-slate-100 bg-gradient-to-br from-blue-50 to-indigo-50 px-5 py-4">
                            <Show when=move || h.price_was.is_some()>
                                <span class="mb-1 inline-flex items-center gap-1 rounded-full bg-red-600 px-2 py-0.5 text-[11px] font-bold text-white">
                                    <Icon name="percent" class="h-2.5 w-2.5" />
                                    {format!("Was ETB {}", thousands(h.price_was.unwrap_or(0)))}
                                </span>
                            </Show>
                            <p class="text-xs font-semibold uppercase tracking-wide text-slate-500">"Rooms from"</p>
                            <p class="text-3xl font-extrabold tracking-tight text-slate-900">
                                {format!("ETB {}", thousands(h.price_from))}
                                <span class="text-sm font-medium text-slate-500">" / night"</span>
                            </p>
                        </div>

                        <div class="p-5">
                            <ul class="flex flex-col gap-2.5 text-sm">
                                {[
                                    ("shield-check", "No booking fees"),
                                    ("wallet", "Pay at the hotel"),
                                    ("check-circle", "Instant confirmation"),
                                    ("users", "No account required"),
                                ].into_iter().map(|(icon, label)| view! {
                                    <li class="flex items-center gap-2 text-slate-700">
                                        <Icon name=icon class="h-4 w-4 shrink-0 text-emerald-600" />
                                        {label}
                                    </li>
                                }).collect_view()}
                            </ul>

                            {match cheapest {
                                Some(r) => view! {
                                    <A
                                        href=format!("/hotels/{}/reserve/{}", h.id, r.id)
                                        attr:class="sheen mt-5 flex w-full items-center justify-center gap-2 rounded-xl bg-blue-700 py-3.5 text-sm font-bold text-white shadow-lg shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-xl active:scale-[0.98]"
                                    >
                                        <Icon name="calendar-check" class="h-4 w-4" />
                                        "Reserve a room"
                                    </A>
                                }.into_any(),
                                None => view! {
                                    <p class="mt-5 rounded-xl bg-slate-50 p-3 text-center text-sm text-slate-500">
                                        "No rooms are loaded for this property yet."
                                    </p>
                                }.into_any(),
                            }}

                            <a
                                href="#rooms"
                                class="mt-2 flex w-full items-center justify-center gap-2 rounded-xl border border-slate-300 py-3 text-sm font-bold text-slate-700 transition-colors hover:border-blue-300 hover:text-blue-700"
                            >
                                "Compare all room types"
                            </a>

                            <div class="mt-5 border-t border-slate-100 pt-5">
                                <h3 class="mb-2.5 text-xs font-bold uppercase tracking-wider text-slate-400">"Contact the hotel"</h3>
                                <div class="flex flex-col gap-2 text-sm">
                                    <a
                                        href=format!("tel:{}", h.phone.replace(' ', ""))
                                        class="flex items-center gap-2.5 rounded-xl border border-slate-200 px-3 py-2.5 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:shadow-sm"
                                    >
                                        <Icon name="phone" class="h-4 w-4 shrink-0 text-blue-700" />
                                        <span class="min-w-0">
                                            <span class="block text-xs text-slate-400">"Call"</span>
                                            <span class="block truncate font-semibold text-slate-800">{h.phone}</span>
                                        </span>
                                    </a>
                                    <a
                                        href=format!("https://wa.me/{}", h.whatsapp.replace([' ', '+'], ""))
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="flex items-center gap-2.5 rounded-xl border border-slate-200 px-3 py-2.5 transition-all duration-200 hover:-translate-y-0.5 hover:border-emerald-300 hover:shadow-sm"
                                    >
                                        <Icon name="message" class="h-4 w-4 shrink-0 text-emerald-600" />
                                        <span class="min-w-0">
                                            <span class="block text-xs text-slate-400">"WhatsApp"</span>
                                            <span class="block truncate font-semibold text-slate-800">{h.whatsapp}</span>
                                        </span>
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="mt-3 flex items-start gap-2.5 rounded-2xl bg-amber-50 p-4 text-xs leading-relaxed text-amber-800 ring-1 ring-amber-100">
                        <Icon name="info" class="mt-0.5 h-4 w-4 shrink-0" />
                        <span>
                            <span class="font-bold">"Rates move quickly. "</span>
                            "Prices shown are for the dates in your search and are confirmed with the hotel at the moment you reserve."
                        </span>
                    </div>
                </aside>
            </div>
        </div>

        // ---- Mobile sticky reserve bar ------------------------------------
        <div class="sticky bottom-14 z-20 border-t border-slate-200 bg-white/95 px-4 py-3 backdrop-blur-lg sm:bottom-0 lg:hidden">
            <div class="mx-auto flex max-w-6xl items-center justify-between gap-3">
                <div>
                    <p class="text-[11px] text-slate-400">"From"</p>
                    <p class="text-lg font-extrabold text-slate-900">{format!("ETB {}", thousands(h.price_from))}</p>
                </div>
                {match cheapest {
                    Some(r) => view! {
                        <A
                            href=format!("/hotels/{}/reserve/{}", h.id, r.id)
                            attr:class="sheen flex items-center gap-2 rounded-xl bg-blue-700 px-6 py-3 text-sm font-bold text-white shadow-lg shadow-blue-700/25 active:scale-[0.98]"
                        >
                            "Reserve now"
                            <Icon name="arrow-right" class="h-4 w-4" />
                        </A>
                    }.into_any(),
                    None => view! { <span></span> }.into_any(),
                }}
            </div>
        </div>
    }
}
