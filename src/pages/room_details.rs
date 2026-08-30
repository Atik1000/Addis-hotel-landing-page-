//! `/hotels/:id/rooms/:room_id` — everything about one room before booking it.
//!
//! Fed by `GET /rooms/public/{id}/`, which unlike the room list carries the
//! room's own policy, the property policy behind it, the floor and any stretch
//! of nights the room is off the market. Dates picked up in the search widget
//! ride through the query string so the "Reserve" button lands on the booking
//! form already filled in.

use crate::api::{get_room, money_round, HotelPolicies, RoomBlock, RoomSummary};
use crate::components::{pluralize, Breadcrumbs, Gallery, Icon};
use crate::images::room_image;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::{use_params_map, use_query_map};

#[component]
pub fn RoomDetailsPage() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();

    let room_id = move || params.get().get("room_id").unwrap_or_default();
    let hotel_id = move || params.get().get("id").unwrap_or_default();
    let dates = move || {
        let q = query.get();
        match (q.get("check_in"), q.get("check_out")) {
            (Some(a), Some(b)) if !a.is_empty() && !b.is_empty() => {
                format!("?check_in={a}&check_out={b}")
            }
            _ => String::new(),
        }
    };

    let room = Resource::new(room_id, |id| async move { get_room(id).await });

    view! {
        <Suspense fallback=|| view! { <RoomSkeleton/> }>
            {move || Suspend::new(async move {
                let hid = hotel_id();
                let d = dates();
                match room.await {
                    Err(e) => view! { <RoomNotFound message=e.to_string() /> }.into_any(),
                    Ok(r) => view! { <RoomBody room=r hotel_id=hid dates=d /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn RoomBody(room: RoomSummary, hotel_id: String, dates: String) -> impl IntoView {
    let hotel = room.hotel.clone();
    let hotel_name = hotel
        .as_ref()
        .map(|h| h.name.clone())
        .unwrap_or_else(|| "This hotel".into());
    let currency = hotel
        .as_ref()
        .and_then(|h| h.currency.clone())
        .unwrap_or_else(|| "ETB".into());
    let city = hotel.as_ref().and_then(|h| h.city.clone());
    // The room's own hotel id is authoritative; the URL is only a hint.
    let hid = hotel
        .as_ref()
        .map(|h| h.id.to_string())
        .filter(|s| s != "0")
        .unwrap_or(hotel_id);

    let price = room.price();
    let was = room.was_price();
    let discount = room.discount();
    let beds = room.bed_summary();
    let bed_count = room.bed_count();
    let capacity = room.guest_capacity.max(1);
    let rules = room.rules();
    let groups = room.amenities_by_category();
    let blocks: Vec<RoomBlock> = room.blocks().into_iter().cloned().collect();
    let bookable = room.is_bookable();
    let room_type = room.room_type.clone().unwrap_or_default();
    let floor = room.floor;

    // Room photos where the hotel has uploaded them, curated stand-ins where it
    // has not, so the mosaic is never a grey box.
    let own: Vec<String> = {
        let mut p = room.photos.clone();
        p.sort_by_key(|x| x.display_order);
        p.into_iter()
            .map(|x| x.image_url)
            .filter(|u| u.starts_with("http"))
            .collect()
    };
    let photos: Vec<String> = if own.is_empty() {
        // Five, so the mosaic's hero-plus-four grid fills completely.
        (0..5)
            .map(|i| {
                room_image(
                    room.id + i * 7,
                    None,
                    if i == 0 { room.room_type.as_deref() } else { None },
                    1400,
                )
            })
            .collect()
    } else {
        own
    };

    let reserve_href = format!("/hotels/{hid}/reserve/{}{dates}", room.id);
    let hotel_href = format!("/hotels/{hid}{dates}");

    let trail = vec![
        ("Home".to_string(), Some("/".to_string())),
        ("Hotels".to_string(), Some("/hotels".to_string())),
        (
            city.clone().unwrap_or_else(|| "Ethiopia".into()),
            Some(format!(
                "/hotels?city={}",
                city.clone().unwrap_or_default().replace(' ', "+")
            )),
        ),
        (hotel_name.clone(), Some(hotel_href.clone())),
        (room.name.clone(), None),
    ];

    let title = format!("{} · {hotel_name}", room.name);

    view! {
        <Title text=title />

        <div class="mx-auto max-w-6xl px-4 py-5">
            <Breadcrumbs trail=trail />

            <div class="mt-4 animate-fade-up">
                <Gallery photos=photos alt=room.name.clone() />
            </div>

            <div class="mt-7 grid gap-8 lg:grid-cols-[minmax(0,1fr)_21rem]">
                // ================= MAIN COLUMN =================
                <div class="min-w-0">
                    <div class="flex flex-wrap items-center gap-2">
                        <Show when={
                            let t = room_type.clone();
                            move || !t.is_empty()
                        }>
                            <span class="rounded-full bg-blue-50 px-3 py-1 text-[11px] font-bold uppercase tracking-wider text-blue-700 ring-1 ring-blue-100">
                                {room_type.clone()}
                            </span>
                        </Show>
                        {discount.map(|d| view! {
                            <span class="rounded-full bg-red-50 px-3 py-1 text-[11px] font-bold uppercase tracking-wider text-red-700 ring-1 ring-red-100">
                                {format!("{d}% off")}
                            </span>
                        })}
                        <span class=format!(
                            "rounded-full px-3 py-1 text-[11px] font-bold uppercase tracking-wider ring-1 {}",
                            if bookable {
                                "bg-emerald-50 text-emerald-700 ring-emerald-100"
                            } else {
                                "bg-slate-100 text-slate-500 ring-slate-200"
                            }
                        )>
                            {if bookable { "Available" } else { "Not bookable" }}
                        </span>
                    </div>

                    <h1 class="mt-3 text-2xl font-bold tracking-tight text-ink sm:text-3xl">
                        {room.name.clone()}
                    </h1>
                    <p class="mt-1.5 flex flex-wrap items-center gap-x-2 gap-y-1 text-sm text-slate-500">
                        <Icon name="building" class="h-4 w-4 shrink-0" />
                        <a href=hotel_href.clone() class="font-semibold text-blue-700 hover:underline">
                            {hotel_name.clone()}
                        </a>
                        {city.clone().map(|c| view! { <span>{format!("· {c}")}</span> })}
                    </p>

                    // ---- At a glance ----------------------------------------
                    <div class="mt-6 grid grid-cols-2 gap-3 sm:grid-cols-4">
                        <Fact icon="users" label="Sleeps" value=pluralize(capacity, "guest") />
                        <Fact
                            icon="bed"
                            label="Beds"
                            value={if beds.is_empty() {
                                pluralize(bed_count.max(1), "bed")
                            } else {
                                beds.clone()
                            }}
                        />
                        <Fact icon="home" label="Room" value=format!("No. {}", room.room_number) />
                        <Fact
                            icon="list"
                            label="Floor"
                            value=floor.map(|f| f.to_string()).unwrap_or_else(|| "—".into())
                        />
                    </div>

                    // ---- What's included ------------------------------------
                    <div class="mt-5 flex flex-wrap gap-2">
                        {room.breakfast_included.then(|| view! {
                            <span class="inline-flex items-center gap-1.5 rounded-xl bg-emerald-50 px-3 py-2 text-xs font-semibold text-emerald-700 ring-1 ring-emerald-100">
                                <Icon name="coffee" class="h-3.5 w-3.5" />
                                "Breakfast included"
                            </span>
                        })}
                        <span class="inline-flex items-center gap-1.5 rounded-xl bg-slate-50 px-3 py-2 text-xs font-semibold text-slate-600 ring-1 ring-slate-200">
                            <Icon name="wallet" class="h-3.5 w-3.5" />
                            "Pay at the hotel"
                        </span>
                        <span class="inline-flex items-center gap-1.5 rounded-xl bg-slate-50 px-3 py-2 text-xs font-semibold text-slate-600 ring-1 ring-slate-200">
                            <Icon name="shield-check" class="h-3.5 w-3.5" />
                            "No booking fee"
                        </span>
                    </div>

                    // ---- Amenities ------------------------------------------
                    <Show when={
                        let g = groups.clone();
                        move || !g.is_empty()
                    }>
                        <section class="mt-9">
                            <h2 class="text-lg font-bold text-ink">"What this room offers"</h2>
                            <div class="mt-4 grid gap-5 sm:grid-cols-2">
                                {groups.clone().into_iter().map(|(cat, items)| view! {
                                    <div class="rounded-2xl border border-slate-200 bg-white p-4">
                                        <p class="text-[11px] font-bold uppercase tracking-wider text-slate-400">
                                            {cat}
                                        </p>
                                        <ul class="mt-2.5 flex flex-col gap-2">
                                            {items.into_iter().map(|a| view! {
                                                <li class="flex items-start gap-2 text-sm text-slate-600">
                                                    <Icon name="check" class="mt-0.5 h-4 w-4 shrink-0 text-emerald-600" />
                                                    {a}
                                                </li>
                                            }).collect_view()}
                                        </ul>
                                    </div>
                                }).collect_view()}
                            </div>
                        </section>
                    </Show>

                    // ---- Sleeping arrangement -------------------------------
                    <Show when={
                        let b = room.beds.clone();
                        move || !b.is_empty()
                    }>
                        <section class="mt-9">
                            <h2 class="text-lg font-bold text-ink">"Sleeping arrangement"</h2>
                            <div class="mt-4 flex flex-wrap gap-3">
                                {room.beds.clone().into_iter().map(|b| {
                                    let n = b.number_of_beds.unwrap_or(1);
                                    let t = b.bed_type.clone().unwrap_or_else(|| "Bed".into());
                                    view! {
                                        <div class="flex min-w-[9rem] flex-col gap-1 rounded-2xl border border-slate-200 bg-white px-4 py-3">
                                            <Icon name="bed" class="h-5 w-5 text-slate-400" />
                                            <span class="text-sm font-bold text-ink">{t}</span>
                                            <span class="text-xs text-slate-500">
                                                {if n == 1 { "1 bed".to_string() } else { format!("{n} beds") }}
                                            </span>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </section>
                    </Show>

                    // ---- House rules ----------------------------------------
                    <RoomRules rules=rules.clone() />

                    // ---- Blocked dates --------------------------------------
                    <Show when={
                        let b = blocks.clone();
                        move || !b.is_empty()
                    }>
                        <section class="mt-9">
                            <h2 class="text-lg font-bold text-ink">"Dates this room is unavailable"</h2>
                            <ul class="mt-3 flex flex-col gap-2">
                                {blocks.clone().into_iter().map(|b| view! {
                                    <li class="flex items-center gap-2.5 rounded-xl border border-amber-200 bg-amber-50 px-4 py-2.5 text-sm text-amber-900">
                                        <Icon name="calendar" class="h-4 w-4 shrink-0" />
                                        <span class="font-semibold">
                                            {format!("{} → {}", b.event_start_date, b.event_finished_date)}
                                        </span>
                                        <span class="text-amber-700">
                                            {b.event_type.clone().unwrap_or_else(|| "Unavailable".into())}
                                        </span>
                                    </li>
                                }).collect_view()}
                            </ul>
                        </section>
                    </Show>

                    <A
                        href=hotel_href.clone()
                        attr:class="mt-9 inline-flex items-center gap-1.5 text-sm font-bold text-blue-700 transition-colors hover:text-blue-800"
                    >
                        <Icon name="arrow-left" class="h-4 w-4" />
                        {format!("All rooms at {hotel_name}")}
                    </A>
                </div>

                // ================= BOOKING CARD =================
                <aside class="lg:sticky lg:top-24 lg:self-start">
                    <div class="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm">
                        <div class="border-b border-slate-100 p-5">
                            {was.map(|w| view! {
                                <p class="text-sm text-slate-400 line-through">
                                    {format!("{currency} {}", money_round(w))}
                                </p>
                            })}
                            <p class="flex items-baseline gap-1.5">
                                <span class="text-3xl font-bold tracking-tight text-ink">
                                    {format!("{currency} {}", money_round(price))}
                                </span>
                                <span class="text-sm text-slate-500">"/ night"</span>
                            </p>
                            <p class="mt-1 text-xs text-slate-400">
                                {format!("Sleeps up to {}", pluralize(capacity, "guest"))}
                            </p>
                        </div>

                        <div class="flex flex-col gap-2.5 p-5">
                            {if bookable {
                                view! {
                                    <A
                                        href=reserve_href.clone()
                                        attr:class="sheen flex w-full items-center justify-center gap-2 rounded-xl bg-blue-700 py-3.5 text-sm font-bold text-white shadow-md shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-lg active:scale-[0.98]"
                                    >
                                        "Reserve this room"
                                        <Icon name="arrow-right" class="h-4 w-4" />
                                    </A>
                                }.into_any()
                            } else {
                                view! {
                                    <span class="flex w-full cursor-not-allowed items-center justify-center gap-2 rounded-xl bg-slate-100 py-3.5 text-sm font-bold text-slate-400">
                                        "Not available"
                                    </span>
                                }.into_any()
                            }}
                            <A
                                href=hotel_href.clone()
                                attr:class="flex w-full items-center justify-center gap-2 rounded-xl border border-slate-300 py-3 text-sm font-bold text-slate-700 transition-colors hover:border-blue-300 hover:text-blue-700"
                            >
                                "See other rooms"
                            </A>

                            <ul class="mt-2 flex flex-col gap-2 text-xs text-slate-500">
                                <li class="flex items-center gap-2">
                                    <Icon name="check-circle" class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                    "No card details needed"
                                </li>
                                <li class="flex items-center gap-2">
                                    <Icon name="check-circle" class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                    "Pay at the front desk"
                                </li>
                                <li class="flex items-center gap-2">
                                    <Icon name="clock" class="h-3.5 w-3.5 shrink-0 text-slate-400" />
                                    {format!("Check-in from {}", rules.checkin_display())}
                                </li>
                            </ul>
                        </div>
                    </div>
                </aside>
            </div>
        </div>
    }
}

/// House rules for the room, falling back to the property's where the room sets
/// nothing of its own.
#[component]
fn RoomRules(rules: HotelPolicies) -> impl IntoView {
    let checkin = rules.checkin_display();
    let checkout = rules.checkout_display();

    // `Option<bool>` — `None` means the hotel never answered, and an unanswered
    // rule is left off rather than shown as a prohibition.
    let flags: Vec<(&str, &str, Option<bool>)> = vec![
        ("users", "Children welcome", rules.children_allowed),
        ("plus", "Extra bed available", rules.extrabed_available),
        ("heart", "Pets allowed", rules.pet_allowed),
        ("wine", "Smoking allowed", rules.smoking_allowed),
        ("sparkles", "Parties or events allowed", rules.parties_or_event_allowed),
        ("scan", "Government ID required", rules.government_id_required),
    ];
    let shown: Vec<(&str, &str, bool)> = flags
        .into_iter()
        .filter_map(|(i, l, v)| v.map(|v| (i, l, v)))
        .collect();
    let note = rules.public_note.clone().filter(|n| !n.trim().is_empty());

    view! {
        <section class="mt-9">
            <h2 class="text-lg font-bold text-ink">"House rules"</h2>
            <div class="mt-4 grid gap-3 sm:grid-cols-2">
                <div class="flex items-center gap-3 rounded-2xl border border-slate-200 bg-white p-4">
                    <span class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-blue-50 text-blue-700">
                        <Icon name="clock" class="h-5 w-5" />
                    </span>
                    <span>
                        <span class="block text-[11px] uppercase tracking-wide text-slate-400">"Check-in"</span>
                        <span class="block text-sm font-bold text-ink">{checkin}</span>
                    </span>
                </div>
                <div class="flex items-center gap-3 rounded-2xl border border-slate-200 bg-white p-4">
                    <span class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-blue-50 text-blue-700">
                        <Icon name="clock" class="h-5 w-5" />
                    </span>
                    <span>
                        <span class="block text-[11px] uppercase tracking-wide text-slate-400">"Check-out"</span>
                        <span class="block text-sm font-bold text-ink">{checkout}</span>
                    </span>
                </div>
            </div>

            <Show when={
                let s = shown.clone();
                move || !s.is_empty()
            }>
                <ul class="mt-3 grid gap-2 sm:grid-cols-2">
                    {shown.clone().into_iter().map(|(icon, label, allowed)| view! {
                        <li class="flex items-center gap-2.5 rounded-xl border border-slate-200 bg-white px-4 py-2.5 text-sm">
                            <Icon
                                name=if allowed { "check-circle" } else { "x-circle" }
                                class=if allowed {
                                    "h-4 w-4 shrink-0 text-emerald-600"
                                } else {
                                    "h-4 w-4 shrink-0 text-slate-300"
                                }
                            />
                            <span class=if allowed { "text-slate-700" } else { "text-slate-400 line-through" }>
                                {label}
                            </span>
                            <Icon name=icon class="ml-auto h-3.5 w-3.5 text-slate-300" />
                        </li>
                    }).collect_view()}
                </ul>
            </Show>

            {note.map(|n| view! {
                <p class="mt-3 rounded-xl border border-slate-200 bg-slate-50 px-4 py-3 text-sm leading-relaxed text-slate-600">
                    {n}
                </p>
            })}
        </section>
    }
}

#[component]
fn Fact(icon: &'static str, label: &'static str, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2.5 rounded-xl border border-slate-200 bg-white p-3">
            <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-slate-100 text-slate-600">
                <Icon name=icon class="h-4 w-4" />
            </span>
            <span class="min-w-0">
                <span class="block text-[11px] uppercase tracking-wide text-slate-400">{label}</span>
                <span class="block truncate text-sm font-bold text-slate-800">{value}</span>
            </span>
        </div>
    }
}

#[component]
fn RoomNotFound(#[prop(into)] message: String) -> impl IntoView {
    view! {
        <div class="mx-auto max-w-md px-4 py-24 text-center">
            <span class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-slate-100 text-slate-400">
                <Icon name="bed" class="h-6 w-6" />
            </span>
            <h1 class="text-xl font-bold text-ink">"Room not found"</h1>
            <p class="mt-2 text-sm text-slate-500">{message}</p>
            <A href="/hotels" attr:class="mt-5 inline-flex rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white hover:bg-blue-800">
                "Browse all hotels"
            </A>
        </div>
    }
}

#[component]
fn RoomSkeleton() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-6xl px-4 py-5">
            <div class="skeleton h-[22rem] rounded-2xl sm:h-[26rem]"></div>
            <div class="mt-7 grid gap-8 lg:grid-cols-[minmax(0,1fr)_21rem]">
                <div class="flex flex-col gap-4">
                    <div class="skeleton h-8 w-2/3 rounded-lg"></div>
                    <div class="skeleton h-4 w-1/2 rounded-lg"></div>
                    <div class="skeleton h-20 rounded-xl"></div>
                    <div class="skeleton h-40 rounded-2xl"></div>
                </div>
                <div class="skeleton h-72 rounded-2xl"></div>
            </div>
        </div>
    }
}
