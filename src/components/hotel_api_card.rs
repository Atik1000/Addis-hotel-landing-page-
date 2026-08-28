//! Hotel cards for the listings page.
//!
//! `GET /organizations/public/` returns only identity, location, star rating and
//! a logo — no nightly price, guest score or gallery. The "from" price therefore
//! arrives separately, summarised from the room feed by
//! [`crate::api::hotel_from_prices`], and is passed in rather than fetched per
//! card.

use crate::api::{money_round, HotelSummary};
use crate::components::{FavoriteButton, Icon, Stars};
use leptos::prelude::*;
use leptos_router::components::A;

/// Neutral placeholder for a hotel with no logo set.
#[component]
fn LogoOrFallback(hotel: HotelSummary, #[prop(into)] class: String) -> impl IntoView {
    let initial = hotel
        .name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".into());

    view! {
        {match hotel.logo.as_deref().filter(|s| !s.trim().is_empty()) {
            Some(src) => view! {
                <img
                    src=src.to_string()
                    alt=hotel.name.clone()
                    loading="lazy"
                    class=format!("{class} object-cover")
                />
            }.into_any(),
            None => view! {
                <div class=format!(
                    "{class} flex items-center justify-center bg-gradient-to-br \
                     from-blue-600 to-indigo-700 text-2xl font-extrabold text-white"
                )>
                    {initial}
                </div>
            }.into_any(),
        }}
    }
}

/// Full-width row used by the listings page.
#[component]
pub fn HotelApiCard(
    hotel: HotelSummary,
    /// Cheapest nightly rate, when the room feed had one for this hotel.
    #[prop(default = None)]
    from_price: Option<f64>,
    /// Query string carrying the searched dates through to the hotel page.
    #[prop(into, default = String::new())]
    dates: String,
) -> impl IntoView {
    let href = format!("/hotels/{}{dates}", hotel.id);
    let currency = hotel.currency.clone().unwrap_or_else(|| "ETB".into());
    let stars = hotel.stars();
    let location = hotel.location();
    let blurb = hotel
        .description
        .clone()
        .filter(|d| !d.trim().is_empty())
        .map(|d| d.chars().take(160).collect::<String>());

    view! {
        <article class="card-hover group flex flex-col overflow-hidden rounded-2xl border border-slate-200 bg-white sm:flex-row">
            <div class="relative h-44 w-full shrink-0 overflow-hidden sm:h-auto sm:w-56">
                <LogoOrFallback hotel=hotel.clone() class="h-full w-full" />
                <div class="absolute right-2 top-2">
                    <FavoriteButton />
                </div>
            </div>

            <div class="flex flex-1 flex-col gap-4 p-4 sm:flex-row">
                <div class="min-w-0 flex-1">
                    <Show when=move || (stars > 0.0)>
                        <div class="flex items-center gap-1.5">
                            <Stars rating=stars class="h-3 w-3" />
                            <span class="text-[11px] font-bold uppercase tracking-wide text-slate-400">
                                {format!("{}-star hotel", stars.round() as u32)}
                            </span>
                        </div>
                    </Show>

                    <h3 class="mt-0.5 text-lg font-bold text-slate-900">{hotel.name.clone()}</h3>

                    <Show when={
                        let l = location.clone();
                        move || !l.is_empty()
                    }>
                        <p class="mt-1 flex items-center gap-1 text-sm text-slate-500">
                            <Icon name="map-pin" class="h-3.5 w-3.5 shrink-0" />
                            {location.clone()}
                        </p>
                    </Show>

                    {blurb.clone().map(|b| view! {
                        <p class="mt-2 line-clamp-2 text-sm leading-relaxed text-slate-500">{b}</p>
                    })}
                </div>

                <div class="flex shrink-0 flex-row items-end justify-between gap-3 border-slate-100 sm:w-44 sm:flex-col sm:items-end sm:justify-center sm:border-l sm:pl-4">
                    {match from_price {
                        Some(p) => view! {
                            <div class="sm:text-right">
                                <p class="text-[11px] text-slate-400">"from"</p>
                                <p class="text-xl font-extrabold text-slate-900">
                                    {format!("{currency} {}", money_round(p))}
                                </p>
                                <p class="text-[11px] text-slate-400">"per night"</p>
                            </div>
                        }.into_any(),
                        None => view! {
                            <p class="text-xs text-slate-400">"See rooms"</p>
                        }.into_any(),
                    }}
                    <A
                        href=href.clone()
                        attr:class="sheen flex items-center gap-1.5 rounded-xl bg-blue-700 px-4 py-2.5 text-sm font-bold text-white shadow-md transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-lg active:scale-95"
                    >
                        "View Details"
                        <Icon name="arrow-right" class="h-4 w-4" />
                    </A>
                </div>
            </div>
        </article>
    }
}

/// Square tile used by the grid toggle.
#[component]
pub fn HotelApiCardCompact(
    hotel: HotelSummary,
    #[prop(default = None)] from_price: Option<f64>,
    #[prop(into, default = String::new())] dates: String,
) -> impl IntoView {
    let href = format!("/hotels/{}{dates}", hotel.id);
    let currency = hotel.currency.clone().unwrap_or_else(|| "ETB".into());
    let stars = hotel.stars();
    let location = hotel.location();

    view! {
        <A href=href attr:class="card-hover group flex flex-col overflow-hidden rounded-2xl border border-slate-200 bg-white">
            <div class="relative h-36 w-full overflow-hidden">
                <LogoOrFallback hotel=hotel.clone() class="h-full w-full" />
            </div>
            <div class="flex flex-1 flex-col p-3">
                <Show when=move || (stars > 0.0)>
                    <Stars rating=stars class="h-3 w-3" />
                </Show>
                <h3 class="mt-1 truncate text-sm font-bold text-slate-900">{hotel.name.clone()}</h3>
                <p class="mt-0.5 truncate text-xs text-slate-500">{location}</p>
                {from_price.map(|p| view! {
                    <p class="mt-1.5 text-sm font-extrabold text-slate-900">
                        {format!("{currency} {}", money_round(p))}
                        <span class="text-[11px] font-medium text-slate-400">" / night"</span>
                    </p>
                })}
            </div>
        </A>
    }
}
