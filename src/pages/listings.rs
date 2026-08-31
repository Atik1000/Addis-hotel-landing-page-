//! Hotel browsing, backed by `GET /organizations/public/`.
//!
//! Filtering, sorting and pagination all happen on the API — the page holds
//! filter state, turns it into a query, and renders whatever comes back.
//!
//! The endpoint returns identity, location, star rating and a logo. It carries
//! no nightly price, guest score or review count, so the filter rail offers
//! only what the API can actually apply; a control that silently does nothing
//! is worse than no control. Price and guest-rating filters can come back the
//! moment those fields exist upstream.

use crate::api::{
    hotel_from_prices, list_amenities, list_cities, list_hotels, Amenity, City, HotelQuery,
};
use crate::components::{
    EmptyState, HotelApiCard, HotelApiCardCompact, Icon, SearchWidget, SkeletonCard,
};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;

const PER_PAGE: u32 = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sort {
    Recommended,
    StarsHigh,
    StarsLow,
    NameAsc,
    Newest,
}

impl Sort {
    fn label(self) -> &'static str {
        match self {
            Sort::Recommended => "Recommended",
            Sort::StarsHigh => "Star class: high to low",
            Sort::StarsLow => "Star class: low to high",
            Sort::NameAsc => "Name: A to Z",
            Sort::Newest => "Recently added",
        }
    }

    /// The `ordering` value the API expects; `None` leaves it to the default.
    fn ordering(self) -> Option<&'static str> {
        match self {
            Sort::Recommended => None,
            Sort::StarsHigh => Some("-star_rating"),
            Sort::StarsLow => Some("star_rating"),
            Sort::NameAsc => Some("name"),
            Sort::Newest => Some("-created_at"),
        }
    }

    const ALL: [Sort; 5] = [
        Sort::Recommended,
        Sort::StarsHigh,
        Sort::StarsLow,
        Sort::NameAsc,
        Sort::Newest,
    ];
}

#[component]
pub fn ListingsPage() -> impl IntoView {
    let query = use_query_map();

    // ---- Filter state ----------------------------------------------------
    let city = RwSignal::new(String::new());
    let text = RwSignal::new(String::new());
    let star_classes = RwSignal::new(Vec::<u32>::new());
    let amenity_ids = RwSignal::new(Vec::<i64>::new());
    let sort = RwSignal::new(Sort::Recommended);
    let grid_view = RwSignal::new(false);
    let page = RwSignal::new(1u32);
    let filters_open = RwSignal::new(false);
    let sort_open = RwSignal::new(false);

    // Seed the city filter from `?city=` so a search from the home page lands
    // on pre-filtered results.
    // Dates arrive from the search widget as `check_in` / `check_out`; the
    // hotels endpoint filters by availability when both are present, and they
    // are carried through to the hotel page so its room list matches.
    let check_in = RwSignal::new(String::new());
    let check_out = RwSignal::new(String::new());

    Effect::new(move |_| {
        let q = query.get();
        if let Some(c) = q.get("city") {
            let c = c.replace('+', " ");
            if !c.is_empty() {
                city.set(c);
            }
        }
        // Free text from the search box arrives as `search=`; a city chosen from
        // the typeahead or a destination tile arrives as `city=`. Both have to be
        // honoured, or a search for a hotel by name lands on an unfiltered page.
        if let Some(t) = q.get("search") {
            let t = t.replace('+', " ");
            if !t.is_empty() {
                text.set(t);
            }
        }
        check_in.set(q.get("check_in").unwrap_or_default());
        check_out.set(q.get("check_out").unwrap_or_default());
    });

    // ---- Reference data --------------------------------------------------
    let cities = Resource::new(|| (), |_| async move { list_cities(Some(100)).await });
    // `GET /organizations/public/` has no price field, so one pass over the room
    // feed supplies the "from" figure for every card on the page.
    let prices = Resource::new(|| (), |_| async move { hotel_from_prices().await });
    let amenities = Resource::new(|| (), |_| async move { list_amenities().await });

    // ---- Results ---------------------------------------------------------
    let hotels = Resource::new(
        move || {
            (
                city.get(),
                text.get(),
                star_classes.get(),
                amenity_ids.get(),
                sort.get().ordering().map(str::to_owned),
                page.get(),
                check_in.get(),
                check_out.get(),
            )
        },
        |(city, search, stars, amenities, ordering, page, check_in, check_out)| async move {
            list_hotels(HotelQuery {
                search: Some(search),
                city: Some(city),
                // The endpoint takes a numeric range, so a discrete set of star
                // classes collapses to its bounds; the exact set is applied
                // again on the rows that come back.
                //
                // The upper bound is `+0.9`, not `+0.99`: the API rounds
                // max_rating to one decimal, so 4.99 became 5.0 and the 4-star
                // chip returned 5-star hotels.
                min_rating: stars.iter().min().map(|s| *s as f32),
                max_rating: stars.iter().max().map(|s| *s as f32 + 0.9),
                amenities: (!amenities.is_empty()).then(|| {
                    amenities
                        .iter()
                        .map(i64::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                }),
                ordering,
                page: Some(page),
                page_size: Some(PER_PAGE),
                check_in: Some(check_in),
                check_out: Some(check_out),
                ..Default::default()
            })
            .await
        },
    );

    let toggle_star = move |s: u32| {
        star_classes.update(|list| match list.iter().position(|x| *x == s) {
            Some(i) => {
                list.remove(i);
            }
            None => list.push(s),
        });
        page.set(1);
    };

    let toggle_amenity = move |id: i64| {
        amenity_ids.update(|list| match list.iter().position(|x| *x == id) {
            Some(i) => {
                list.remove(i);
            }
            None => list.push(id),
        });
        page.set(1);
    };

    let clear_all = move |_| {
        city.set(String::new());
        text.set(String::new());
        star_classes.set(Vec::new());
        amenity_ids.set(Vec::new());
        page.set(1);
    };

    let active_count = move || {
        let mut n = 0;
        if !city.get().is_empty() {
            n += 1;
        }
        if !text.get().is_empty() {
            n += 1;
        }
        n + star_classes.get().len() + amenity_ids.get().len()
    };

    // Counts and pagination follow the API's `meta`, not a local tally. Every
    // reader goes through `Suspend`/`await` rather than `resource.get()`: a
    // bare closure escapes its Suspense boundary even when it looks nested
    // inside one, which Leptos warns causes hydration mismatches.

    let heading = move || {
        let c = city.get();
        if !c.is_empty() {
            return format!("Hotels in {c}");
        }
        let t = text.get();
        if !t.is_empty() {
            return format!("Hotels matching \u{201c}{t}\u{201d}");
        }
        "Hotels across the Horn of Africa".to_string()
    };

    // Mirror the amenity catalogue into a plain signal. Chip labels are read
    // outside any Suspense boundary, and an effect is the one reader Leptos
    // allows there; chips only appear after a client-side click anyway.
    let amenity_list = RwSignal::new(Vec::<Amenity>::new());
    Effect::new(move |_| {
        if let Some(Ok(list)) = amenities.get() {
            amenity_list.set(list);
        }
    });
    let amenity_name = move |id: i64| {
        amenity_list
            .get()
            .iter()
            .find(|a| a.id == id)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| format!("Amenity {id}"))
    };

    view! {
        <Title text="Browse Hotels — Horn of Africa Hotel Portal" />

        // `relative z-30` is load-bearing. The filter rail below is
        // `lg:sticky`, which makes it a positioned element, while the search
        // widget sits inside `animate-fade-up` whose transform creates a
        // stacking context with `z-index: auto`. A positioned element paints
        // above such a context, so the rail's first card punched through the
        // open destination dropdown and hid the suggestions. Positioning this
        // block lifts the widget and its panels above the rail. It stays below
        // the site header's z-40 so that still wins while scrolling.
        <div class="relative z-30 border-b border-slate-200 bg-white">
            <div class="mx-auto max-w-6xl px-4 py-5">
                <div class="mb-4 flex flex-wrap items-end justify-between gap-3">
                    <div class="animate-fade-up">
                        <h1 class="text-2xl font-bold tracking-tight text-ink">{heading}</h1>
                        <p class="mt-1 flex items-center gap-1.5 text-sm text-slate-500">
                            <Icon name="check-circle" class="h-3.5 w-3.5 text-emerald-600" />
                            <Suspense fallback=|| view! { <span>"Searching…"</span> }>
                                {move || Suspend::new(async move {
                                    let n = hotels.await.map(|p| p.meta.count).unwrap_or(0);
                                    format!(
                                        "{n} verified {} match your search",
                                        if n == 1 { "property" } else { "properties" },
                                    )
                                })}
                            </Suspense>
                        </p>
                    </div>
                </div>
                <div class="animate-fade-up" style="animation-delay: 80ms">
                    <SearchWidget horizontal=true />
                </div>
            </div>
        </div>

        <div class="mx-auto max-w-6xl gap-7 px-4 py-6 lg:grid lg:grid-cols-[17rem_minmax(0,1fr)]">
            // ================= FILTER RAIL =================
            <aside class=move || format!(
                "{} lg:block",
                if filters_open.get() { "fixed inset-0 z-50 overflow-y-auto bg-white p-4 lg:static lg:z-auto lg:overflow-visible lg:bg-transparent lg:p-0" } else { "hidden" }
            )>
                <div class="lg:sticky lg:top-24">
                    <div class="mb-3 flex items-center justify-between lg:hidden">
                        <h2 class="text-lg font-bold text-ink">"Filters"</h2>
                        <button
                            class="flex h-9 w-9 items-center justify-center rounded-lg text-slate-500 hover:bg-slate-100"
                            on:click=move |_| filters_open.set(false)
                        >
                            <Icon name="x" class="h-5 w-5" />
                        </button>
                    </div>

                    <div class="flex flex-col gap-3">
                        <FilterCard title="Search by name" icon="search">
                            <div class="relative">
                                <Icon name="search" class="pointer-events-none absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
                                <input
                                    type="text"
                                    placeholder="Hotel, area or city"
                                    class="w-full rounded-lg border border-slate-300 py-2 pl-8 pr-3 text-sm transition-colors focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100"
                                    prop:value=text
                                    on:change:target=move |ev| { text.set(ev.target().value()); page.set(1); }
                                />
                            </div>
                        </FilterCard>

                        <FilterCard title="City" icon="map-pin">
                            <div class="flex flex-col gap-1">
                                <FilterRadio
                                    label="All cities".to_string()
                                    checked=Signal::derive(move || city.get().is_empty())
                                    on_pick=Callback::new(move |_| { city.set(String::new()); page.set(1); })
                                />
                                <Suspense fallback=|| view! { <p class="px-2 py-1.5 text-sm text-slate-400">"Loading cities…"</p> }>
                                    {move || Suspend::new(async move {
                                        let list: Vec<City> = cities.await.map(|p| p.items).unwrap_or_default();
                                        list.into_iter().map(|c| {
                                            let name = c.city.clone();
                                            let pick = c.city.clone();
                                            let check = c.city.clone();
                                            view! {
                                                <FilterRadio
                                                    label=name
                                                    checked=Signal::derive(move || city.get() == check)
                                                    on_pick=Callback::new(move |_| { city.set(pick.clone()); page.set(1); })
                                                />
                                            }
                                        }).collect_view()
                                    })}
                                </Suspense>
                            </div>
                        </FilterCard>

                        <FilterCard title="Star class" icon="star">
                            <div class="flex flex-wrap gap-1.5">
                                {[5u32, 4, 3].into_iter().map(|s| view! {
                                    <button
                                        on:click=move |_| toggle_star(s)
                                        class=move || format!(
                                            "flex items-center gap-1 rounded-lg border px-2.5 py-1.5 text-xs font-semibold transition-all duration-200 active:scale-95 {}",
                                            if star_classes.get().contains(&s) {
                                                "border-blue-600 bg-blue-50 text-blue-700"
                                            } else {
                                                "border-slate-300 text-slate-600 hover:border-blue-300 hover:bg-slate-50"
                                            }
                                        )
                                    >
                                        {s}
                                        <Icon name="star" class="h-3 w-3 text-amber-500" />
                                    </button>
                                }).collect_view()}
                            </div>
                        </FilterCard>

                        <FilterCard title="Amenities" icon="sliders">
                            <Suspense fallback=|| view! { <p class="text-sm text-slate-400">"Loading amenities…"</p> }>
                                {move || Suspend::new(async move {
                                    let list: Vec<Amenity> = amenities.await.unwrap_or_default();
                                    if list.is_empty() {
                                        return view! {
                                            <p class="text-sm text-slate-400">"No amenities published yet."</p>
                                        }.into_any();
                                    }
                                    view! {
                                        <div class="flex max-h-56 flex-col gap-0.5 overflow-y-auto pr-1">
                                            {list.into_iter().map(|a| {
                                                let id = a.id;
                                                view! {
                                                    <button
                                                        on:click=move |_| toggle_amenity(id)
                                                        class="flex items-center gap-2.5 rounded-lg px-2 py-1.5 text-left text-sm text-slate-600 transition-colors hover:bg-slate-50"
                                                    >
                                                        <span class=move || format!(
                                                            "flex h-4 w-4 shrink-0 items-center justify-center rounded border-2 transition-colors {}",
                                                            if amenity_ids.get().contains(&id) {
                                                                "border-blue-700 bg-blue-700 text-white"
                                                            } else {
                                                                "border-slate-300"
                                                            }
                                                        )>
                                                            <Show when=move || amenity_ids.get().contains(&id)>
                                                                <Icon name="check" class="h-2.5 w-2.5" />
                                                            </Show>
                                                        </span>
                                                        <span class="truncate">{a.name}</span>
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                })}
                            </Suspense>
                        </FilterCard>

                        <button
                            on:click=clear_all
                            class="rounded-xl border border-slate-300 px-4 py-2.5 text-sm font-semibold text-slate-600 transition-colors hover:bg-slate-50"
                        >
                            "Clear all filters"
                        </button>
                    </div>

                    <div class="mt-4 lg:hidden">
                        <button
                            on:click=move |_| filters_open.set(false)
                            class="w-full rounded-xl bg-blue-700 px-5 py-3 text-sm font-bold text-white"
                        >
                            <Transition fallback=|| view! { "Show hotels" }>
                                {move || Suspend::new(async move {
                                    let n = hotels.await.map(|p| p.meta.count).unwrap_or(0);
                                    format!("Show {n} hotels")
                                })}
                            </Transition>
                        </button>
                    </div>
                </div>
            </aside>

            // ================= RESULTS =================
            <div class="min-w-0">
                <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
                    <div class="flex items-center gap-2">
                        <button
                            on:click=move |_| filters_open.set(true)
                            class="flex items-center gap-2 rounded-xl border border-slate-300 bg-white px-3.5 py-2 text-sm font-semibold text-slate-700 lg:hidden"
                        >
                            <Icon name="sliders" class="h-4 w-4" />
                            "Filters"
                            <Show when=move || (active_count() > 0)>
                                <span class="flex h-5 min-w-5 items-center justify-center rounded-full bg-blue-700 px-1 text-[11px] font-bold text-white">
                                    {active_count}
                                </span>
                            </Show>
                        </button>

                        <div class="relative">
                            <button
                                on:click=move |_| sort_open.update(|v| *v = !*v)
                                class="flex items-center gap-2 rounded-xl border border-slate-300 bg-white px-3.5 py-2 text-sm font-semibold text-slate-700 transition-colors hover:bg-slate-50"
                            >
                                <Icon name="sort" class="h-4 w-4 text-slate-400" />
                                {move || sort.get().label()}
                                <Icon name="chevron-down" class="h-3.5 w-3.5 text-slate-400" />
                            </button>
                            <Show when=move || sort_open.get()>
                                <div class="absolute left-0 top-full z-20 mt-1.5 w-56 animate-scale-in rounded-xl border border-slate-200 bg-white p-1.5 shadow-xl">
                                    {Sort::ALL.into_iter().map(|s| view! {
                                        <button
                                            on:click=move |_| { sort.set(s); sort_open.set(false); page.set(1); }
                                            class=move || format!(
                                                "flex w-full items-center justify-between rounded-lg px-3 py-2 text-left text-sm transition-colors {}",
                                                if sort.get() == s { "bg-blue-50 font-semibold text-blue-700" } else { "text-slate-600 hover:bg-slate-50" }
                                            )
                                        >
                                            {s.label()}
                                            <Show when=move || (sort.get() == s)>
                                                <Icon name="check" class="h-3.5 w-3.5" />
                                            </Show>
                                        </button>
                                    }).collect_view()}
                                </div>
                            </Show>
                        </div>
                    </div>

                    <div class="flex items-center gap-3">
                        <span class="hidden text-sm text-slate-500 sm:block">
                            <Transition fallback=|| view! { "" }>
                            {move || Suspend::new(async move {
                                let m = hotels.await.map(|p| p.meta).unwrap_or_default();
                                if m.count == 0 { String::new() } else {
                                    let first = (m.page.saturating_sub(1)) * PER_PAGE + 1;
                                    let last = (first + PER_PAGE - 1).min(m.count);
                                    format!("{first}–{last} of {}", m.count)
                                }
                            })}
                            </Transition>
                        </span>
                        <div class="flex overflow-hidden rounded-lg border border-slate-300">
                            <button
                                aria-label="List view"
                                on:click=move |_| grid_view.set(false)
                                class=move || format!("flex h-8 w-8 items-center justify-center transition-colors {}", if grid_view.get() { "text-slate-400 hover:bg-slate-50" } else { "bg-blue-700 text-white" })
                            >
                                <Icon name="list" class="h-4 w-4" />
                            </button>
                            <button
                                aria-label="Grid view"
                                on:click=move |_| grid_view.set(true)
                                class=move || format!("flex h-8 w-8 items-center justify-center transition-colors {}", if grid_view.get() { "bg-blue-700 text-white" } else { "text-slate-400 hover:bg-slate-50" })
                            >
                                <Icon name="grid" class="h-4 w-4" />
                            </button>
                        </div>
                    </div>
                </div>

                // ---- Active filter chips ------------------------------------
                <Show when=move || (active_count() > 0)>
                    <div class="mb-4 flex flex-wrap items-center gap-2">
                        <Show when=move || !city.get().is_empty()>
                            <Chip
                                label=Signal::derive(move || city.get())
                                on_clear=Callback::new(move |_| { city.set(String::new()); page.set(1); })
                            />
                        </Show>
                        <Show when=move || !text.get().is_empty()>
                            <Chip
                                label=Signal::derive(move || format!("\"{}\"", text.get()))
                                on_clear=Callback::new(move |_| { text.set(String::new()); page.set(1); })
                            />
                        </Show>
                        {move || star_classes.get().into_iter().map(|s| view! {
                            <Chip
                                label=Signal::derive(move || format!("{s} star"))
                                on_clear=Callback::new(move |_| toggle_star(s))
                            />
                        }).collect_view()}
                        {move || amenity_ids.get().into_iter().map(|id| view! {
                            <Chip
                                label=Signal::derive(move || amenity_name(id))
                                on_clear=Callback::new(move |_| toggle_amenity(id))
                            />
                        }).collect_view()}
                    </div>
                </Show>

                // ---- Result list --------------------------------------------
                <Transition fallback=move || view! {
                    <div class="flex flex-col gap-4">
                        {(0..3).map(|_| view! { <SkeletonCard/> }).collect_view()}
                    </div>
                }>
                    {move || Suspend::new(async move {
                        // Awaited, not read: a plain `.get()` here resolves to
                        // `None` on the first paint and the cards lose their price.
                        let from_prices = prices.await.unwrap_or_default();
                        // The API's rating filter is a range, so a non-contiguous
                        // selection (5 and 3) still brings back the classes in
                        // between. Narrow to the exact chips, matching on the same
                        // rounded value the cards display.
                        let picked = star_classes.get();
                        let keep = move |h: &crate::api::HotelSummary| {
                            picked.is_empty() || picked.contains(&(h.stars().round() as u32))
                        };
                        let hotels_result = hotels.await.map(|mut p| {
                            p.items.retain(&keep);
                            p
                        });
                        match hotels_result {
                            Err(e) => view! {
                                <EmptyState
                                    icon="alert"
                                    title="We couldn't reach the hotel service"
                                    body="The listing service did not respond. Please try again in a moment."
                                >
                                    <p class="max-w-lg break-words rounded-lg bg-slate-50 px-3 py-2 font-mono text-xs text-slate-500">
                                        {e.to_string()}
                                    </p>
                                </EmptyState>
                            }.into_any(),
                            Ok(pageful) if pageful.items.is_empty() => view! {
                                <EmptyState
                                    icon="search"
                                    title="No hotels match those filters"
                                    body="Try removing an amenity, widening the star class, or searching a different city."
                                >
                                    <button
                                        on:click=clear_all
                                        class="rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800"
                                    >
                                        "Clear all filters"
                                    </button>
                                </EmptyState>
                            }.into_any(),
                            Ok(pageful) => view! {
                                <div class=move || if grid_view.get() {
                                    "grid gap-5 sm:grid-cols-2 xl:grid-cols-3"
                                } else {
                                    "flex flex-col gap-4"
                                }>
                                    {
                                        let from = from_prices;
                                        let dates = date_query(&check_in.get(), &check_out.get());
                                        pageful.items.into_iter().enumerate().map(|(i, h)| {
                                            let delay = format!("animation-delay: {}ms", i * 60);
                                            let compact = h.clone();
                                            let price = from.iter().find(|(id, _)| *id == h.id).map(|(_, p)| *p);
                                            let d1 = dates.clone();
                                            let d2 = dates.clone();
                                            view! {
                                                <div class="animate-fade-up" style=delay>
                                                    <Show
                                                        when=move || grid_view.get()
                                                        fallback={
                                                            let h = h.clone();
                                                            let d = d1.clone();
                                                            move || view! {
                                                                <HotelApiCard hotel=h.clone() from_price=price dates=d.clone() />
                                                            }
                                                        }
                                                    >
                                                        <HotelApiCardCompact hotel=compact.clone() from_price=price dates=d2.clone() />
                                                    </Show>
                                                </div>
                                            }
                                        }).collect_view()
                                    }
                                </div>
                            }.into_any(),
                        }
                    })}
                </Transition>

                // ---- Pagination ---------------------------------------------
                // Rendered from an awaited `meta` rather than a reactive read:
                // `<Show when=…>` re-evaluates its predicate in its own scope,
                // which escapes an enclosing Transition and trips Leptos's
                // hydration-mismatch warning.
                <Transition fallback=|| ()>
                    {move || Suspend::new(async move {
                        let m = hotels.await.map(|p| p.meta).unwrap_or_default();
                        let pages = m.pages.max(1);
                        (pages > 1).then(|| view! {
                            <div class="mt-8 flex items-center justify-center gap-1.5 text-sm">
                                <button
                                    aria-label="Previous page"
                                    class="flex h-9 w-9 items-center justify-center rounded-lg border border-slate-300 text-slate-600 transition-colors hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-40"
                                    disabled=move || (page.get() <= 1)
                                    on:click=move |_| page.update(|p| *p = (*p).saturating_sub(1).max(1))
                                >
                                    <Icon name="chevron-left" class="h-4 w-4" />
                                </button>

                                {(1..=pages).map(|i| view! {
                                    <button
                                        on:click=move |_| page.set(i)
                                        class=move || format!(
                                            "flex h-9 min-w-9 items-center justify-center rounded-lg px-3 font-semibold transition-all duration-200 {}",
                                            if page.get() == i {
                                                "bg-blue-700 text-white shadow-md shadow-blue-700/25"
                                            } else {
                                                "border border-slate-300 text-slate-600 hover:-translate-y-0.5 hover:bg-slate-50"
                                            }
                                        )
                                    >
                                        {i}
                                    </button>
                                }).collect_view()}

                                <button
                                    aria-label="Next page"
                                    class="flex h-9 w-9 items-center justify-center rounded-lg border border-slate-300 text-slate-600 transition-colors hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-40"
                                    disabled=move || (page.get() >= pages)
                                    on:click=move |_| page.update(|p| *p += 1)
                                >
                                    <Icon name="chevron-right" class="h-4 w-4" />
                                </button>
                            </div>
                        })
                    })}
                </Transition>

                <div class="mt-10 flex flex-col items-center gap-2 rounded-2xl border border-blue-100 bg-blue-50/60 px-6 py-7 text-center">
                    <Icon name="headset" class="h-7 w-7 text-blue-700" />
                    <p class="font-bold text-slate-800">"Can't find the right hotel?"</p>
                    <p class="max-w-md text-sm text-slate-500">
                        "Tell us your city, dates and budget and our team will suggest properties that fit — usually within a couple of hours."
                    </p>
                    <A
                        href="/contact"
                        attr:class="mt-2 rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-lg"
                    >
                        "Ask our team"
                    </A>
                </div>
            </div>
        </div>
    }
}

#[component]
fn FilterCard(title: &'static str, icon: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="rounded-xl border border-slate-200 bg-white p-3.5">
            <h3 class="mb-2.5 flex items-center gap-1.5 text-xs font-bold uppercase tracking-wider text-slate-500">
                <Icon name=icon class="h-3.5 w-3.5 text-slate-400" />
                {title}
            </h3>
            {children()}
        </div>
    }
}

#[component]
fn FilterRadio(
    label: String,
    #[prop(into)] checked: Signal<bool>,
    on_pick: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| on_pick.run(())
            class=move || format!(
                "flex items-center gap-2.5 rounded-lg px-2 py-1.5 text-left text-sm transition-colors {}",
                if checked.get() { "bg-blue-50 font-semibold text-blue-700" } else { "text-slate-600 hover:bg-slate-50" }
            )
        >
            <span class=move || format!(
                "flex h-4 w-4 shrink-0 items-center justify-center rounded-full border-2 transition-colors {}",
                if checked.get() { "border-blue-700" } else { "border-slate-300" }
            )>
                <Show when=move || checked.get()>
                    <span class="h-2 w-2 animate-pop-in rounded-full bg-blue-700"></span>
                </Show>
            </span>
            <span class="truncate">{label.clone()}</span>
        </button>
    }
}

#[component]
fn Chip(#[prop(into)] label: Signal<String>, on_clear: Callback<()>) -> impl IntoView {
    view! {
        <span class="flex animate-pop-in items-center gap-1.5 rounded-full bg-blue-50 py-1 pl-3 pr-1.5 text-xs font-semibold text-blue-700 ring-1 ring-blue-100">
            {move || label.get()}
            <button
                aria-label="Remove filter"
                class="flex h-4 w-4 items-center justify-center rounded-full bg-blue-200/70 text-blue-800 transition-colors hover:bg-blue-300"
                on:click=move |_| on_clear.run(())
            >
                <Icon name="x" class="h-2.5 w-2.5" />
            </button>
        </span>
    }
}

/// The dates the guest searched, as a query string to append to a hotel link,
/// so its room list opens filtered to the same range.
fn date_query(check_in: &str, check_out: &str) -> String {
    if check_in.is_empty() || check_out.is_empty() {
        return String::new();
    }
    format!("?check_in={check_in}&check_out={check_out}")
}
