use crate::components::{Icon, RatingBadge};
use crate::data::Hotel;
use leptos::prelude::*;
use leptos_router::components::A;

pub fn tag_icon(tag: &str) -> &'static str {
    match tag {
        "Wi-Fi" | "Free Wi-Fi" => "wifi",
        "Breakfast" => "coffee",
        "Airport Shuttle" | "Airport Pickup" => "shuttle",
        "Parking" => "shuttle",
        "Pool" | "Swimming Pool" => "waves",
        "Gym" => "dumbbell",
        "Spa" => "sparkles",
        "Bar" => "wine",
        "Restaurant" => "utensils",
        "Beach" => "waves",
        "Security" => "shield-check",
        "24/7 Front Desk" => "clock",
        "Room Service" => "bed",
        "Laundry" => "shirt-off",
        "Family Rooms" => "users",
        "Wheelchair Access" => "check-circle",
        "Air Conditioning" => "snowflake",
        "Business Centre" => "briefcase",
        _ => "check",
    }
}

/// Small favourite toggle used on every image tile.
#[component]
pub fn FavoriteButton(#[prop(default = "h-4 w-4")] size: &'static str) -> impl IntoView {
    let on = RwSignal::new(false);
    view! {
        <button
            aria-label="Save hotel"
            on:click=move |ev| {
                ev.prevent_default();
                ev.stop_propagation();
                on.update(|v| *v = !*v);
            }
            class="absolute right-2.5 top-2.5 z-10 flex h-8 w-8 items-center justify-center rounded-full bg-white/95 text-slate-400 shadow-md backdrop-blur transition-all duration-200 hover:scale-110 hover:text-red-500 active:scale-90"
        >
            {move || view! {
                <Icon
                    name=if on.get() { "heart-fill" } else { "heart" }
                    class=if on.get() { "h-4 w-4 text-red-500 animate-pop-in" } else { size }
                />
            }}
        </button>
    }
}

/// Horizontal card used in the listings results column.
#[component]
pub fn HotelCard(hotel: &'static Hotel) -> impl IntoView {
    view! {
        <article class="card-hover group flex flex-col overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm hover:border-blue-200 sm:flex-row">
            <div class="skeleton relative h-48 w-full shrink-0 overflow-hidden sm:h-auto sm:w-60">
                <img
                    src=hotel.image
                    alt=hotel.name
                    loading="lazy"
                    class="h-full w-full object-cover transition-transform duration-700 group-hover:scale-105"
                />
                <FavoriteButton />
                <Show when=move || hotel.price_was.is_some()>
                    <span class="absolute left-2.5 top-2.5 flex items-center gap-1 rounded-full bg-red-600 px-2.5 py-1 text-[11px] font-bold text-white shadow-md">
                        <Icon name="percent" class="h-3 w-3" />
                        "Deal"
                    </span>
                </Show>
            </div>

            <div class="flex flex-1 flex-col gap-4 p-4 sm:flex-row">
                <div class="min-w-0 flex-1">
                    <div class="flex items-center gap-1.5">
                        <span class="flex items-center gap-0.5 text-amber-500">
                            {(0..hotel.star_class).map(|_| view! { <Icon name="star" class="h-3 w-3" /> }).collect_view()}
                        </span>
                        <span class="text-[11px] font-semibold uppercase tracking-wide text-slate-400">
                            {format!("{}-star hotel", hotel.star_class)}
                        </span>
                    </div>

                    <h3 class="mt-1 truncate text-lg font-bold text-slate-900 transition-colors group-hover:text-blue-700">
                        {hotel.name}
                    </h3>

                    <p class="mt-1 flex items-center gap-1 text-sm text-slate-500">
                        <Icon name="map-pin" class="h-3.5 w-3.5 shrink-0" />
                        <span class="truncate">{format!("{}, {}, {}", hotel.area, hotel.city, hotel.country)}</span>
                    </p>

                    <div class="mt-2.5">
                        <RatingBadge rating=hotel.rating review_count=hotel.review_count />
                    </div>

                    <div class="mt-3 flex flex-wrap gap-1.5">
                        {hotel.tags.iter().map(|t| view! {
                            <span class="flex items-center gap-1 rounded-full bg-slate-100 px-2.5 py-1 text-[11px] font-medium text-slate-600 transition-colors group-hover:bg-blue-50 group-hover:text-blue-700">
                                <Icon name=tag_icon(t) class="h-3 w-3" />
                                {*t}
                            </span>
                        }).collect_view()}
                    </div>
                </div>

                <div class="flex shrink-0 items-end justify-between gap-3 border-t border-slate-100 pt-3 sm:w-40 sm:flex-col sm:items-end sm:justify-between sm:border-l sm:border-t-0 sm:pl-4 sm:pt-0">
                    <div class="sm:text-right">
                        <Show when=move || hotel.price_was.is_some()>
                            <p class="text-xs text-slate-400 line-through">
                                {format!("ETB {}", hotel.price_was.unwrap_or(0))}
                            </p>
                        </Show>
                        <p class="text-xs text-slate-400">"From"</p>
                        <p class="text-xl font-extrabold tracking-tight text-slate-900">
                            {format!("ETB {}", thousands(hotel.price_from))}
                        </p>
                        <p class="text-xs text-slate-400">"per night"</p>
                    </div>
                    <A
                        href=format!("/hotels/{}", hotel.id)
                        attr:class="sheen flex items-center gap-1.5 rounded-xl bg-blue-700 px-4 py-2.5 text-sm font-semibold text-white shadow-md shadow-blue-700/20 transition-all duration-200 hover:bg-blue-800 hover:shadow-lg active:scale-[0.98] sm:w-full sm:justify-center"
                    >
                        "View Details"
                        <Icon name="arrow-right" class="h-3.5 w-3.5" />
                    </A>
                </div>
            </div>
        </article>
    }
}

/// Vertical card used in the grid view and the "featured" carousel.
#[component]
pub fn HotelCardCompact(hotel: &'static Hotel) -> impl IntoView {
    view! {
        <A href=format!("/hotels/{}", hotel.id) attr:class="block h-full">
            <article class="card-hover group flex h-full flex-col overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm hover:border-blue-200">
                <div class="skeleton relative h-44 shrink-0 overflow-hidden">
                    <img
                        src=hotel.image
                        alt=hotel.name
                        loading="lazy"
                        class="h-full w-full object-cover transition-transform duration-700 group-hover:scale-110"
                    />
                    <span class="absolute inset-0 bg-gradient-to-t from-slate-900/55 via-transparent to-transparent"></span>
                    <FavoriteButton />
                    <Show when=move || hotel.price_was.is_some()>
                        <span class="absolute left-2.5 top-2.5 rounded-full bg-red-600 px-2.5 py-1 text-[11px] font-bold text-white shadow-md">
                            {format!(
                                "-{}%",
                                100u32.saturating_sub(hotel.price_from * 100 / hotel.price_was.unwrap_or(hotel.price_from).max(1))
                            )}
                        </span>
                    </Show>
                    <span class="absolute bottom-2.5 left-3 flex items-center gap-1 text-xs font-medium text-white/90">
                        <Icon name="map-pin" class="h-3 w-3" />
                        {format!("{}, {}", hotel.area, hotel.city)}
                    </span>
                </div>

                <div class="flex flex-1 flex-col p-4">
                    <div class="flex items-start justify-between gap-2">
                        <h3 class="line-clamp-2 text-sm font-bold leading-snug text-slate-900 transition-colors group-hover:text-blue-700">
                            {hotel.name}
                        </h3>
                        <span class="flex shrink-0 items-center gap-1 rounded-lg bg-amber-100 px-1.5 py-0.5 text-xs font-bold text-amber-700">
                            <Icon name="star" class="h-3 w-3" />
                            {format!("{:.1}", hotel.rating)}
                        </span>
                    </div>

                    <p class="mt-1.5 line-clamp-1 text-xs text-slate-500">{hotel.highlight}</p>

                    <div class="mt-3 flex flex-wrap gap-1">
                        {hotel.tags.iter().take(3).map(|t| view! {
                            <span class="flex items-center gap-1 rounded-md bg-slate-100 px-1.5 py-0.5 text-[10px] font-medium text-slate-500">
                                <Icon name=tag_icon(t) class="h-2.5 w-2.5" />
                                {*t}
                            </span>
                        }).collect_view()}
                    </div>

                    <div class="mt-auto flex items-end justify-between pt-4">
                        <span>
                            <span class="block text-[11px] text-slate-400">{pluralize(hotel.review_count, "review")}</span>
                            <span class="block text-lg font-extrabold text-slate-900">
                                {format!("ETB {}", thousands(hotel.price_from))}
                            </span>
                        </span>
                        <span class="flex h-9 w-9 items-center justify-center rounded-full bg-blue-50 text-blue-700 transition-all duration-300 group-hover:bg-blue-700 group-hover:text-white">
                            <Icon name="arrow-right" class="h-4 w-4" />
                        </span>
                    </div>
                </div>
            </article>
        </A>
    }
}

/// `(1, "guest")` -> `"1 guest"`, `(3, "guest")` -> `"3 guests"`.
///
/// Only handles nouns that pluralise with a bare `s`, which covers every count
/// noun on the site (guest, room, night, review).
pub fn pluralize(n: u32, noun: &str) -> String {
    if n == 1 {
        format!("{n} {noun}")
    } else {
        format!("{n} {noun}s")
    }
}

/// `12345` -> `"12,345"`.
pub fn thousands(n: u32) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out.chars().rev().collect()
}
