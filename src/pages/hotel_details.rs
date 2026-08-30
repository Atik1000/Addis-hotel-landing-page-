//! One hotel, backed by `GET /organizations/public/{id}/`.
//!
//! Three server functions feed the page: the hotel record (identity, policies,
//! contacts, gallery), its bookable rooms from `/rooms/public/`, and its guest
//! reviews from `/reviews/`. All three are `Resource`s so the page is rendered
//! on the server and arrives complete for search engines.
//!
//! The fixtures this page used to show — square metres, "only N left", nearby
//! landmarks, refundable flags — have no counterpart in the API, so the page
//! now shows what the property actually publishes instead.

use crate::api::{
    get_hotel_detail, list_hotel_reviews, list_hotel_rooms, money_round, HotelDetail, HotelReviews,
    RoomSummary,
};
use crate::components::{pluralize, Breadcrumbs, Disclosure, Gallery, Icon, Stars};
use crate::images::{hotel_gallery, room_image};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::{use_params_map, use_query_map};

#[component]
pub fn HotelDetailsPage() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();

    // Dates carried over from the search widget so the room list can be filtered
    // to what is actually free.
    let check_in = move || query.get().get("check_in").unwrap_or_default();
    let check_out = move || query.get().get("check_out").unwrap_or_default();
    let id = move || params.get().get("id").unwrap_or_default();

    let hotel = Resource::new(id, |id| async move { get_hotel_detail(id).await });
    let rooms = Resource::new(
        move || (id(), check_in(), check_out()),
        |(id, ci, co)| async move { list_hotel_rooms(id, Some(ci), Some(co)).await },
    );
    let reviews = Resource::new(id, |id| async move { list_hotel_reviews(id).await });

    view! {
        <Suspense fallback=|| view! { <HotelSkeleton/> }>
            {move || Suspend::new(async move {
                match hotel.await {
                    Err(_) | Ok(_) if false => ().into_any(),
                    Err(e) => view! { <NotFound message=e.to_string() /> }.into_any(),
                    Ok(h) => view! { <HotelBody hotel=h rooms=rooms reviews=reviews /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn HotelBody(
    hotel: HotelDetail,
    rooms: Resource<Result<Vec<RoomSummary>, ServerFnError>>,
    reviews: Resource<Result<HotelReviews, ServerFnError>>,
) -> impl IntoView {
    let description_expanded = RwSignal::new(false);
    let show_all_amenities = RwSignal::new(false);
    let reviews_shown = RwSignal::new(3usize);

    let h = hotel.clone();
    let currency = h.currency_code().to_string();
    let stars = h.stars();
    let star_class = stars.round() as u32;
    let location = h.location();
    let city_line = h.city_line();
    let policies = h.policies.clone().unwrap_or_default();
    // Falls back to curated photography — every hotel in the feed currently
    // has an empty photo list, and a monogram reads as a broken page.
    let gallery = hotel_gallery(h.id, h.gallery());
    let contacts = h.contacts.clone();
    let amenities = h.amenities.clone();

    let map_url = format!(
        "https://www.google.com/maps/search/?api=1&query={}",
        location.replace(' ', "+")
    );

    let trail = vec![
        ("Home".to_string(), Some("/".to_string())),
        ("Hotels".to_string(), Some("/hotels".to_string())),
        (
            h.city.clone().unwrap_or_else(|| "Ethiopia".into()),
            Some(format!(
                "/hotels?city={}",
                h.city.clone().unwrap_or_default().replace(' ', "+")
            )),
        ),
        (h.name.clone(), None),
    ];

    let hotel_id = h.id;
    let name = h.name.clone();
    let description = h.description.clone().unwrap_or_default();
    let has_description = !description.trim().is_empty();
    let description_len = description.len();
    let checkin_display = policies.checkin_display();
    let checkout_display = policies.checkout_display();
    let house_rules = policies.rules();
    let payment_methods = policies.payment_methods();
    let public_note = policies.public_note.clone();
    let description_text = StoredValue::new(description);
    // The rooms block re-renders, so the currency cannot be moved into it.
    let currency_code = StoredValue::new(currency.clone());

    view! {
        <Title text=format!("{} — {} | Horn of Africa Hotel Portal", name, city_line) />

        <div class="mx-auto max-w-6xl px-4 py-5">
            <div class="mb-4 animate-fade-up">
                <Breadcrumbs trail=trail />
            </div>

            // ---- Gallery -------------------------------------------------
            <div class="animate-fade-up" style="animation-delay: 60ms">
                <Gallery photos=gallery alt=name.clone() />
            </div>

            <div class="mt-7 grid gap-8 lg:grid-cols-[minmax(0,1fr)_21rem]">
                // ================= MAIN COLUMN =================
                <div class="min-w-0">
                    <div class="animate-fade-up" style="animation-delay: 100ms">
                        <div class="flex flex-wrap items-center gap-2">
                            <Show when=move || (star_class > 0)>
                                <span class="flex items-center gap-0.5 text-amber-500">
                                    {(0..star_class).map(|_| view! { <Icon name="star" class="h-3.5 w-3.5" /> }).collect_view()}
                                </span>
                                <span class="text-[11px] font-bold uppercase tracking-wider text-slate-400">
                                    {format!("{star_class}-star hotel")}
                                </span>
                            </Show>
                            <span class="flex items-center gap-1 rounded-full bg-emerald-50 px-2.5 py-0.5 text-[11px] font-bold text-emerald-700 ring-1 ring-emerald-100">
                                <Icon name="shield-check" class="h-3 w-3" />
                                "Verified property"
                            </span>
                        </div>

                        <h1 class="mt-2 text-3xl font-bold tracking-tight text-ink">{name.clone()}</h1>

                        <p class="mt-1.5 flex flex-wrap items-center gap-1.5 text-sm text-slate-500">
                            <Icon name="map-pin" class="h-4 w-4 shrink-0 text-blue-700" />
                            {location.clone()}
                            <a href=map_url target="_blank" rel="noopener noreferrer" class="ml-1 font-semibold text-blue-700 hover:underline">
                                "Show on map"
                            </a>
                        </p>

                        // The rating is the guests' own, straight from the review feed.
                        <Suspense fallback=|| ()>
                            {move || Suspend::new(async move {
                                let r = reviews.await.unwrap_or_default();
                                if r.count == 0 {
                                    return view! {
                                        <p class="mt-3 text-xs text-slate-400">"No guest reviews yet"</p>
                                    }.into_any();
                                }
                                view! {
                                    <div class="mt-3 flex items-center gap-2">
                                        <span class="rounded-lg bg-blue-700 px-2 py-1 text-sm font-bold text-white">
                                            {format!("{:.1}", r.average)}
                                        </span>
                                        <Stars rating=r.average class="h-3.5 w-3.5" />
                                        <span class="text-xs text-slate-500">{pluralize(r.count, "review")}</span>
                                    </div>
                                }.into_any()
                            })}
                        </Suspense>
                    </div>

                    // ---- Quick facts ------------------------------------
                    <div class="mt-6 grid animate-fade-up grid-cols-2 gap-3 sm:grid-cols-4" style="animation-delay: 140ms">
                        <Fact icon="clock" label="Check-in" value=policies.checkin_display() />
                        <Fact icon="clock" label="Check-out" value=policies.checkout_display() />
                        <Fact
                            icon="bed"
                            label="Rooms"
                            value=Signal::derive(move || match rooms.get() {
                                Some(Ok(list)) => format!("{} available", list.len()),
                                _ => "Loading…".to_string(),
                            })
                        />
                        <Fact icon="wallet" label="Payment" value="At the hotel" />
                    </div>

                    // ---- About ------------------------------------------
                    <Show when=move || has_description>
                        <section class="mt-8">
                            <h2 class="text-lg font-bold text-ink">"About this hotel"</h2>
                            <p class=move || format!(
                                "mt-2 text-sm leading-relaxed text-slate-600 transition-all {}",
                                if description_expanded.get() { "" } else { "line-clamp-4" }
                            )>
                                {move || description_text.get_value()}
                            </p>
                            <Show when=move || (description_len > 260)>
                                <button
                                    class="mt-1.5 text-sm font-semibold text-blue-700 hover:underline"
                                    on:click=move |_| description_expanded.update(|v| *v = !*v)
                                >
                                    {move || if description_expanded.get() { "Show less" } else { "Read more" }}
                                </button>
                            </Show>
                        </section>
                    </Show>

                    // ---- Amenities ---------------------------------------
                    <Show when={
                        let n = amenities.len();
                        move || n > 0
                    }>
                        <section class="mt-8">
                            <h2 class="text-lg font-bold text-ink">"Amenities"</h2>
                            <p class="mt-1 text-sm text-slate-500">"What this property offers its guests."</p>
                            <div class="mt-3 flex flex-wrap gap-2">
                                {
                                    let list = amenities.clone();
                                    move || {
                                        let limit = if show_all_amenities.get() { list.len() } else { 12.min(list.len()) };
                                        list.iter().take(limit).map(|a| view! {
                                            <span class="flex items-center gap-1.5 rounded-lg border border-slate-200 bg-white px-3 py-1.5 text-sm text-slate-700">
                                                <Icon name="check" class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                                {a.name.clone()}
                                            </span>
                                        }).collect_view()
                                    }
                                }
                            </div>
                            <Show when={
                                let n = amenities.len();
                                move || n > 12
                            }>
                                <button
                                    class="mt-2 text-sm font-semibold text-blue-700 hover:underline"
                                    on:click=move |_| show_all_amenities.update(|v| *v = !*v)
                                >
                                    {move || if show_all_amenities.get() { "Show fewer".to_string() } else { "Show all amenities".to_string() }}
                                </button>
                            </Show>
                        </section>
                    </Show>

                    // ---- Rooms -------------------------------------------
                    <section class="mt-9" id="rooms">
                        <h2 class="text-lg font-bold text-ink">"Choose your room"</h2>
                        <p class="mt-1 text-sm text-slate-500">
                            "All rates are per room per night and paid at the hotel."
                        </p>

                        <Suspense fallback=|| view! {
                            <div class="mt-4 flex flex-col gap-4">
                                <div class="skeleton h-40 rounded-2xl"></div>
                                <div class="skeleton h-40 rounded-2xl"></div>
                            </div>
                        }>
                            {move || Suspend::new(async move {
                                let list = match rooms.await {
                                    Ok(l) => l,
                                    Err(e) => return view! {
                                        <p class="mt-4 rounded-xl bg-red-50 px-4 py-3 text-sm text-red-700">{e.to_string()}</p>
                                    }.into_any(),
                                };
                                if list.is_empty() {
                                    return view! {
                                        <p class="mt-4 rounded-xl border border-slate-200 bg-white px-4 py-8 text-center text-sm text-slate-500">
                                            "No rooms are available for these dates. Try a different date range."
                                        </p>
                                    }.into_any();
                                }
                                let cur = currency_code.get_value();
                                view! {
                                    <div class="mt-4 flex flex-col gap-4">
                                        {list.into_iter().enumerate().map(|(i, r)| {
                                            view! { <RoomRow room=r hotel_id=hotel_id currency=cur.clone() index=i /> }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            })}
                        </Suspense>
                    </section>

                    // ---- Reviews -----------------------------------------
                    <section class="mt-10">
                        <h2 class="text-lg font-bold text-ink">"Guest reviews"</h2>
                        <Suspense fallback=|| view! {
                            <div class="skeleton mt-4 h-32 rounded-2xl"></div>
                        }>
                            {move || Suspend::new(async move {
                                let r = reviews.await.unwrap_or_default();
                                if r.count == 0 {
                                    return view! {
                                        <p class="mt-4 rounded-2xl border border-slate-200 bg-white px-4 py-8 text-center text-sm text-slate-500">
                                            "No reviews yet. Only guests who have completed a stay can leave one."
                                        </p>
                                    }.into_any();
                                }
                                let total = r.count.max(1);
                                let items = r.items.clone();
                                view! {
                                    <div class="mt-4 grid gap-6 rounded-2xl border border-slate-200 bg-white p-5 sm:grid-cols-[auto_minmax(0,1fr)]">
                                        <div class="flex flex-col items-center justify-center gap-1 border-slate-100 sm:border-r sm:pr-6">
                                            <span class="text-4xl font-bold tracking-tight text-ink">
                                                {format!("{:.1}", r.average)}
                                            </span>
                                            <Stars rating=r.average class="h-4 w-4" />
                                            <span class="text-xs text-slate-400">{pluralize(r.count, "review")}</span>
                                        </div>
                                        <ul class="flex flex-col justify-center gap-1.5">
                                            {r.breakdown.into_iter().enumerate().map(|(i, n)| {
                                                let stars = 5 - i as u32;
                                                let pct = n * 100 / total;
                                                view! {
                                                    <li class="flex items-center gap-2 text-xs">
                                                        <span class="w-8 shrink-0 text-slate-500">{format!("{stars}★")}</span>
                                                        <span class="h-1.5 flex-1 overflow-hidden rounded-full bg-slate-100">
                                                            <span class="block h-full rounded-full bg-amber-400" style=format!("width: {pct}%")></span>
                                                        </span>
                                                        <span class="w-6 shrink-0 text-right tabular-nums text-slate-400">{n}</span>
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    </div>

                                    <div class="mt-4 flex flex-col gap-3">
                                        {
                                            let shown = items.clone();
                                            move || shown.iter().take(reviews_shown.get()).map(|rev| view! {
                                                <article class="rounded-2xl border border-slate-200 bg-white p-4">
                                                    <div class="flex items-start gap-3">
                                                        <span class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-gradient-to-br from-blue-600 to-indigo-700 text-xs font-bold text-white">
                                                            {rev.initials()}
                                                        </span>
                                                        <div class="min-w-0 flex-1">
                                                            <div class="flex flex-wrap items-center gap-2">
                                                                <span class="font-semibold text-ink">{rev.guest_name.clone()}</span>
                                                                <Stars rating=rev.rate as f32 class="h-3 w-3" />
                                                                <span class="text-xs text-slate-400">
                                                                    {crate::api::pretty_date(rev.created_at.as_deref())}
                                                                </span>
                                                            </div>
                                                            {rev.stayed_in().map(|s| view! {
                                                                <p class="mt-0.5 text-xs text-slate-400">{s}</p>
                                                            })}
                                                            <p class="mt-2 text-sm leading-relaxed text-slate-600">{rev.comment.clone()}</p>
                                                            {rev.has_reply().then(|| view! {
                                                                <div class="mt-3 rounded-xl bg-slate-50 p-3">
                                                                    <p class="text-xs font-bold text-slate-700">
                                                                        {format!(
                                                                            "Response from {}",
                                                                            rev.replied_by_name.clone().unwrap_or_else(|| "the hotel".into()),
                                                                        )}
                                                                    </p>
                                                                    <p class="mt-1 text-sm text-slate-600">
                                                                        {rev.reply.clone().unwrap_or_default()}
                                                                    </p>
                                                                </div>
                                                            })}
                                                        </div>
                                                    </div>
                                                </article>
                                            }).collect_view()
                                        }
                                    </div>

                                    {
                                        let n = items.len();
                                        view! {
                                            <Show when=move || (reviews_shown.get() < n)>
                                                <button
                                                    class="mt-3 w-full rounded-xl border border-slate-300 py-2.5 text-sm font-semibold text-slate-700 transition-colors hover:bg-slate-50"
                                                    on:click=move |_| reviews_shown.update(|v| *v += 5)
                                                >
                                                    "Show more reviews"
                                                </button>
                                            </Show>
                                        }
                                    }
                                }.into_any()
                            })}
                        </Suspense>
                    </section>

                    // ---- Policies ----------------------------------------
                    <section class="mt-10">
                        <h2 class="text-lg font-bold text-ink">"Hotel policies"</h2>
                        <div class="mt-3 divide-y divide-slate-100 overflow-hidden rounded-2xl border border-slate-200 bg-white">
                            <Disclosure title="Check-in and check-out" start_open=true>
                                <ul class="flex flex-col gap-2 text-sm text-slate-600">
                                    <li class="flex items-center gap-2">
                                        <Icon name="clock" class="h-4 w-4 shrink-0 text-blue-700" />
                                        {format!("Check-in from {checkin_display}")}
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <Icon name="clock" class="h-4 w-4 shrink-0 text-blue-700" />
                                        {format!("Check-out by {checkout_display}")}
                                    </li>
                                </ul>
                            </Disclosure>

                            {
                                let rules = house_rules;
                                (!rules.is_empty()).then(|| view! {
                                    <Disclosure title="House rules">
                                        <ul class="flex flex-col gap-2 text-sm text-slate-600">
                                            {rules.into_iter().map(|(icon, text)| view! {
                                                <li class="flex items-center gap-2">
                                                    <Icon name=icon class="h-4 w-4 shrink-0 text-blue-700" />
                                                    {text}
                                                </li>
                                            }).collect_view()}
                                        </ul>
                                    </Disclosure>
                                })
                            }

                            <Disclosure title="Payment">
                                <p class="text-sm text-slate-600">
                                    "You pay the hotel directly on arrival — this portal never takes your card details and charges no booking fee."
                                </p>
                                <div class="mt-2 flex flex-wrap gap-2">
                                    {payment_methods.into_iter().map(|m| view! {
                                        <span class="rounded-lg bg-slate-100 px-2.5 py-1 text-xs font-medium text-slate-700">{m}</span>
                                    }).collect_view()}
                                </div>
                            </Disclosure>

                            {public_note.filter(|n| !n.trim().is_empty()).map(|note| view! {
                                <Disclosure title="Good to know">
                                    <p class="whitespace-pre-line text-sm text-slate-600">{note}</p>
                                </Disclosure>
                            })}
                        </div>
                    </section>
                </div>

                // ================= SIDEBAR =================
                <aside class="lg:sticky lg:top-24 lg:self-start">
                    <div class="animate-fade-up rounded-2xl border border-slate-200 bg-white p-5 shadow-sm" style="animation-delay: 180ms">
                        <Suspense fallback=|| view! { <div class="skeleton h-16 rounded-xl"></div> }>
                            {move || Suspend::new(async move {
                                let cheapest = rooms
                                    .await
                                    .ok()
                                    .and_then(|list| {
                                        list.into_iter()
                                            .filter(|r| r.price() > 0.0)
                                            .min_by(|a, b| a.price().total_cmp(&b.price()))
                                    });
                                match cheapest {
                                    Some(r) => view! {
                                        <div>
                                            <p class="text-xs uppercase tracking-wide text-slate-400">"From"</p>
                                            <p class="text-3xl font-bold tracking-tight text-ink">
                                                {format!("ETB {}", money_round(r.price()))}
                                            </p>
                                            <p class="text-xs text-slate-400">"per night, taxes calculated at booking"</p>
                                        </div>
                                    }.into_any(),
                                    None => view! {
                                        <p class="text-sm text-slate-500">"No rooms loaded for these dates."</p>
                                    }.into_any(),
                                }
                            })}
                        </Suspense>

                        <a
                            href="#rooms"
                            class="sheen mt-4 flex w-full items-center justify-center gap-1.5 rounded-xl bg-blue-700 px-4 py-3 text-sm font-bold text-white shadow-md shadow-blue-700/20 transition-all duration-200 hover:bg-blue-800 hover:shadow-lg active:scale-[0.98]"
                        >
                            "Choose a room"
                            <Icon name="arrow-right" class="h-4 w-4" />
                        </a>

                        <ul class="mt-4 flex flex-col gap-2 border-t border-slate-100 pt-4 text-xs text-slate-600">
                            <li class="flex items-center gap-2">
                                <Icon name="check-circle" class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                "No booking fee, ever"
                            </li>
                            <li class="flex items-center gap-2">
                                <Icon name="check-circle" class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                "No account needed"
                            </li>
                            <li class="flex items-center gap-2">
                                <Icon name="check-circle" class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                "Pay the hotel on arrival"
                            </li>
                        </ul>
                    </div>

                    // ---- Contact ------------------------------------------
                    <Show when={
                        let has = h.phone_number.is_some() || h.email.is_some() || !contacts.is_empty();
                        move || has
                    }>
                        <div class="mt-4 rounded-2xl border border-slate-200 bg-white p-5">
                            <h3 class="text-sm font-bold text-ink">"Contact the property"</h3>
                            <ul class="mt-2 flex flex-col gap-2 text-sm text-slate-600">
                                {h.phone_number.clone().filter(|p| !p.is_empty()).map(|p| {
                                    let href = format!("tel:{p}");
                                    view! {
                                        <li class="flex items-center gap-2">
                                            <Icon name="phone" class="h-3.5 w-3.5 shrink-0 text-blue-700" />
                                            <a href=href class="hover:underline">{p}</a>
                                        </li>
                                    }
                                })}
                                {h.whatsapp.clone().filter(|p| !p.is_empty()).map(|p| view! {
                                    <li class="flex items-center gap-2">
                                        <Icon name="message" class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                        <a href=format!("https://wa.me/{}", p.replace([' ', '+', '-'], "")) target="_blank" rel="noopener noreferrer" class="hover:underline">
                                            "WhatsApp"
                                        </a>
                                    </li>
                                })}
                                {h.email.clone().filter(|e| !e.is_empty()).map(|e| {
                                    let href = format!("mailto:{e}");
                                    view! {
                                        <li class="flex items-center gap-2">
                                            <Icon name="mail" class="h-3.5 w-3.5 shrink-0 text-blue-700" />
                                            <a href=href class="truncate hover:underline">{e}</a>
                                        </li>
                                    }
                                })}
                                {h.website_url.clone().filter(|w| w.starts_with("http")).map(|w| view! {
                                    <li class="flex items-center gap-2">
                                        <Icon name="globe" class="h-3.5 w-3.5 shrink-0 text-blue-700" />
                                        <a href=w.clone() target="_blank" rel="noopener noreferrer" class="truncate hover:underline">"Website"</a>
                                    </li>
                                })}
                            </ul>
                            {contacts.iter().find(|c| c.is_primary).map(|c| view! {
                                <p class="mt-3 border-t border-slate-100 pt-3 text-xs text-slate-500">
                                    {format!(
                                        "{}{}",
                                        c.name.clone(),
                                        c.title.clone().map(|t| format!(" · {t}")).unwrap_or_default(),
                                    )}
                                </p>
                            })}
                        </div>
                    </Show>
                </aside>
            </div>
        </div>
    }
}

/// One bookable room.
#[component]
fn RoomRow(room: RoomSummary, hotel_id: i64, currency: String, index: usize) -> impl IntoView {
    let delay = format!("animation-delay: {}ms", index * 70);
    let image = room_image(
        room.id,
        room.primary_image.as_deref(),
        room.room_type.as_deref(),
        500,
    );
    let was = room.was_price();
    let price = room.price();
    let beds = room.bed_summary();
    let amenities: Vec<String> = room.amenities.iter().take(4).map(|a| a.name.clone()).collect();
    let href = format!("/hotels/{hotel_id}/reserve/{}", room.id);
    let detail_href = format!("/hotels/{hotel_id}/rooms/{}", room.id);
    let discount = room.discount_percent_per_night.unwrap_or(0);

    view! {
        <article class="reveal card-hover flex flex-col overflow-hidden rounded-2xl border border-slate-200 bg-white sm:flex-row" style=delay>
            <div class="relative h-44 w-full shrink-0 overflow-hidden sm:h-auto sm:w-56">
                <a href=detail_href.clone() class="block h-full w-full">
                    <img src=image alt=room.name.clone() loading="lazy"
                        class="h-full w-full object-cover transition-transform duration-700 hover:scale-105" />
                </a>
                <Show when=move || (discount > 0)>
                    <span class="absolute bottom-2 left-2 rounded-full bg-red-600 px-2.5 py-1 text-[11px] font-bold text-white shadow-md">
                        {format!("-{discount}%")}
                    </span>
                </Show>
            </div>

            <div class="flex flex-1 flex-col gap-4 p-4 sm:flex-row">
                <div class="min-w-0 flex-1">
                    <h3 class="text-base font-bold text-ink">
                        <a href=detail_href.clone() class="transition-colors hover:text-blue-700">
                            {room.name.clone()}
                        </a>
                    </h3>
                    <p class="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-slate-500">
                        <span class="flex items-center gap-1">
                            <Icon name="bed" class="h-3.5 w-3.5" />
                            {if beds.is_empty() { room.room_type.clone().unwrap_or_default() } else { beds }}
                        </span>
                        <span class="flex items-center gap-1">
                            <Icon name="users" class="h-3.5 w-3.5" />
                            {pluralize(room.guest_capacity, "guest")}
                        </span>
                        <span class="flex items-center gap-1">
                            <Icon name="home" class="h-3.5 w-3.5" />
                            {format!("Room {}", room.room_number)}
                        </span>
                    </p>

                    <ul class="mt-3 flex flex-col gap-1.5">
                        {room.breakfast_included.then(|| view! {
                            <li class="flex items-center gap-1.5 text-xs font-semibold text-emerald-700">
                                <Icon name="coffee" class="h-3.5 w-3.5 shrink-0" />
                                "Breakfast included"
                            </li>
                        })}
                        {amenities.into_iter().map(|a| view! {
                            <li class="flex items-center gap-1.5 text-xs text-slate-600">
                                <Icon name="check" class="h-3.5 w-3.5 shrink-0 text-emerald-600" />
                                {a}
                            </li>
                        }).collect_view()}
                        <li class="flex items-center gap-1.5 text-xs text-slate-500">
                            <Icon name="wallet" class="h-3.5 w-3.5 shrink-0" />
                            "Pay at the hotel"
                        </li>
                    </ul>
                </div>

                <div class="flex shrink-0 items-end justify-between gap-3 border-t border-slate-100 pt-3 sm:w-40 sm:flex-col sm:items-end sm:border-l sm:border-t-0 sm:pl-4 sm:pt-0">
                    <div class="sm:text-right">
                        {was.map(|w| view! {
                            <p class="text-xs text-slate-400 line-through">{format!("{currency} {}", money_round(w))}</p>
                        })}
                        <p class="text-xl font-bold text-ink">
                            {format!("{currency} {}", money_round(price))}
                        </p>
                        <p class="text-xs text-slate-400">"per night"</p>
                    </div>
                    <div class="flex flex-col items-end gap-1.5 sm:w-full">
                        <A
                            href=href
                            attr:class="sheen flex items-center justify-center gap-1.5 rounded-xl bg-blue-700 px-4 py-2.5 text-sm font-bold text-white shadow-md shadow-blue-700/20 transition-all duration-200 hover:bg-blue-800 hover:shadow-lg active:scale-[0.98] sm:w-full"
                        >
                            "Reserve"
                            <Icon name="arrow-right" class="h-3.5 w-3.5" />
                        </A>
                        <A
                            href=detail_href.clone()
                            attr:class="text-xs font-bold text-blue-700 transition-colors hover:text-blue-800 hover:underline"
                        >
                            "View room details"
                        </A>
                    </div>
                </div>
            </div>
        </article>
    }
}

#[component]
fn Fact(
    icon: &'static str,
    label: &'static str,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2.5 rounded-xl border border-slate-200 bg-white p-3">
            <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-slate-100 text-slate-600">
                <Icon name=icon class="h-4 w-4" />
            </span>
            <span class="min-w-0">
                <span class="block text-[11px] uppercase tracking-wide text-slate-400">{label}</span>
                <span class="block truncate text-sm font-bold text-slate-800">{move || value.get()}</span>
            </span>
        </div>
    }
}

#[component]
fn NotFound(#[prop(into)] message: String) -> impl IntoView {
    view! {
        <div class="mx-auto max-w-md px-4 py-24 text-center">
            <span class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-slate-100 text-slate-400">
                <Icon name="search" class="h-6 w-6" />
            </span>
            <h1 class="text-xl font-bold text-ink">"Hotel not found"</h1>
            <p class="mt-2 text-sm text-slate-500">{message}</p>
            <A href="/hotels" attr:class="mt-5 inline-flex rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white hover:bg-blue-800">
                "Browse all hotels"
            </A>
        </div>
    }
}

#[component]
fn HotelSkeleton() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-6xl px-4 py-5">
            <div class="skeleton h-[22rem] rounded-2xl sm:h-[26rem]"></div>
            <div class="mt-7 grid gap-8 lg:grid-cols-[minmax(0,1fr)_21rem]">
                <div class="flex flex-col gap-4">
                    <div class="skeleton h-8 w-2/3 rounded-lg"></div>
                    <div class="skeleton h-4 w-1/2 rounded-lg"></div>
                    <div class="skeleton h-24 rounded-xl"></div>
                    <div class="skeleton h-40 rounded-2xl"></div>
                    <div class="skeleton h-40 rounded-2xl"></div>
                </div>
                <div class="skeleton h-64 rounded-2xl"></div>
            </div>
        </div>
    }
}
