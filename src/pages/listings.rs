use crate::components::{
    tag_icon, thousands, EmptyState, HotelCard, HotelCardCompact, Icon, SearchWidget,
};
use crate::data::{all_amenities, all_cities, Hotel, HOTELS};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;

const PER_PAGE: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sort {
    Recommended,
    PriceLow,
    PriceHigh,
    Rating,
    Stars,
}

impl Sort {
    fn label(self) -> &'static str {
        match self {
            Sort::Recommended => "Recommended",
            Sort::PriceLow => "Price: low to high",
            Sort::PriceHigh => "Price: high to low",
            Sort::Rating => "Guest rating",
            Sort::Stars => "Star class",
        }
    }
    const ALL: [Sort; 5] = [
        Sort::Recommended,
        Sort::PriceLow,
        Sort::PriceHigh,
        Sort::Rating,
        Sort::Stars,
    ];
}

const PRICE_MAX: u32 = 15_000;

#[component]
pub fn ListingsPage() -> impl IntoView {
    let query = use_query_map();

    // ---- Filter state ----------------------------------------------------
    let city = RwSignal::new(String::new());
    let text = RwSignal::new(String::new());
    let max_price = RwSignal::new(PRICE_MAX);
    let min_rating = RwSignal::new(0.0f32);
    let star_classes = RwSignal::new(Vec::<u32>::new());
    let amenities = RwSignal::new(Vec::<String>::new());
    let deals_only = RwSignal::new(false);
    let sort = RwSignal::new(Sort::Recommended);
    let grid_view = RwSignal::new(false);
    let page = RwSignal::new(0usize);
    let filters_open = RwSignal::new(false);
    let sort_open = RwSignal::new(false);

    // Seed the city filter from `?city=` so a search from the home page lands
    // on pre-filtered results.
    Effect::new(move |_| {
        if let Some(c) = query.get().get("city") {
            let c = c.replace('+', " ");
            if !c.is_empty() {
                city.set(c);
            }
        }
    });

    let toggle_star = move |s: u32| {
        star_classes.update(|list| {
            if let Some(i) = list.iter().position(|x| *x == s) {
                list.remove(i);
            } else {
                list.push(s);
            }
        });
        page.set(0);
    };

    let toggle_amenity = move |a: String| {
        amenities.update(|list| {
            if let Some(i) = list.iter().position(|x| *x == a) {
                list.remove(i);
            } else {
                list.push(a);
            }
        });
        page.set(0);
    };

    let clear_all = move |_| {
        city.set(String::new());
        text.set(String::new());
        max_price.set(PRICE_MAX);
        min_rating.set(0.0);
        star_classes.set(Vec::new());
        amenities.set(Vec::new());
        deals_only.set(false);
        page.set(0);
    };

    let active_count = move || {
        let mut n = 0;
        if !city.get().is_empty() { n += 1; }
        if !text.get().is_empty() { n += 1; }
        if max_price.get() < PRICE_MAX { n += 1; }
        if min_rating.get() > 0.0 { n += 1; }
        n += star_classes.get().len();
        n += amenities.get().len();
        if deals_only.get() { n += 1; }
        n
    };

    // ---- Derived results -------------------------------------------------
    let results = Memo::new(move |_| {
        let city_f = city.get().to_lowercase();
        let text_f = text.get().to_lowercase();
        let price_f = max_price.get();
        let rating_f = min_rating.get();
        let stars_f = star_classes.get();
        let amen_f = amenities.get();
        let deals_f = deals_only.get();

        let mut out: Vec<&'static Hotel> = HOTELS
            .iter()
            .filter(|h| city_f.is_empty() || h.city.to_lowercase() == city_f)
            .filter(|h| {
                text_f.is_empty()
                    || h.name.to_lowercase().contains(&text_f)
                    || h.area.to_lowercase().contains(&text_f)
                    || h.city.to_lowercase().contains(&text_f)
            })
            .filter(|h| h.price_from <= price_f)
            .filter(|h| h.rating >= rating_f)
            .filter(|h| stars_f.is_empty() || stars_f.contains(&h.star_class))
            .filter(|h| amen_f.iter().all(|a| h.amenities.contains(&a.as_str())))
            .filter(|h| !deals_f || h.price_was.is_some())
            .collect();

        match sort.get() {
            Sort::Recommended => out.sort_by(|a, b| {
                b.featured
                    .cmp(&a.featured)
                    .then(b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal))
            }),
            Sort::PriceLow => out.sort_by_key(|h| h.price_from),
            Sort::PriceHigh => out.sort_by_key(|h| std::cmp::Reverse(h.price_from)),
            Sort::Rating => out.sort_by(|a, b| {
                b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal)
            }),
            Sort::Stars => out.sort_by_key(|h| std::cmp::Reverse(h.star_class)),
        }
        out
    });

    let page_count = move || results.get().len().div_ceil(PER_PAGE).max(1);
    let visible = move || {
        let all = results.get();
        let start = page.get() * PER_PAGE;
        all.into_iter().skip(start).take(PER_PAGE).collect::<Vec<_>>()
    };

    // Keep the page index inside range when filters shrink the result set.
    Effect::new(move |_| {
        let max = page_count().saturating_sub(1);
        if page.get() > max {
            page.set(max);
        }
    });

    let heading = move || {
        let c = city.get();
        if c.is_empty() { "Hotels across the Horn of Africa".to_string() } else { format!("Hotels in {c}") }
    };

    view! {
        <Title text="Browse Hotels — Horn of Africa Hotel Portal" />

        <div class="border-b border-slate-200 bg-white">
            <div class="mx-auto max-w-6xl px-4 py-5">
                <div class="mb-4 flex flex-wrap items-end justify-between gap-3">
                    <div class="animate-fade-up">
                        <h1 class="text-2xl font-extrabold tracking-tight text-slate-900">{heading}</h1>
                        <p class="mt-1 flex items-center gap-1.5 text-sm text-slate-500">
                            <Icon name="check-circle" class="h-3.5 w-3.5 text-emerald-600" />
                            {move || format!("{} verified properties match your search", results.get().len())}
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
                        <h2 class="text-lg font-bold text-slate-900">"Filters"</h2>
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
                                    placeholder="Hotel or area"
                                    class="w-full rounded-lg border border-slate-300 py-2 pl-8 pr-3 text-sm transition-colors focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100"
                                    prop:value=text
                                    on:input:target=move |ev| { text.set(ev.target().value()); page.set(0); }
                                />
                            </div>
                        </FilterCard>

                        <FilterCard title="City" icon="map-pin">
                            <div class="flex flex-col gap-1">
                                <FilterRadio
                                    label="All cities".to_string()
                                    checked=Signal::derive(move || city.get().is_empty())
                                    on_pick=Callback::new(move |_| { city.set(String::new()); page.set(0); })
                                />
                                {all_cities().into_iter().map(|c| {
                                    let c_owned = c.to_string();
                                    let c_check = c.to_string();
                                    view! {
                                        <FilterRadio
                                            label=c_owned.clone()
                                            checked=Signal::derive(move || city.get() == c_check)
                                            on_pick=Callback::new(move |_| { city.set(c_owned.clone()); page.set(0); })
                                        />
                                    }
                                }).collect_view()}
                            </div>
                        </FilterCard>

                        <FilterCard title="Price per night" icon="tag">
                            <input
                                type="range"
                                min="900"
                                max="15000"
                                step="100"
                                class="w-full accent-blue-700"
                                prop:value=move || max_price.get().to_string()
                                on:input:target=move |ev| {
                                    max_price.set(ev.target().value().parse().unwrap_or(PRICE_MAX));
                                    page.set(0);
                                }
                            />
                            <div class="mt-1.5 flex items-center justify-between text-xs">
                                <span class="text-slate-400">"ETB 900"</span>
                                <span class="rounded-md bg-blue-50 px-2 py-0.5 font-bold text-blue-700 tabular-nums">
                                    {move || format!("up to ETB {}", thousands(max_price.get()))}
                                </span>
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

                        <FilterCard title="Guest rating" icon="smile">
                            <div class="flex flex-col gap-1">
                                {[(0.0f32, "Any rating"), (3.5, "3.5+ Good"), (4.0, "4.0+ Very good"), (4.5, "4.5+ Exceptional")]
                                    .into_iter()
                                    .map(|(value, label)| view! {
                                        <FilterRadio
                                            label=label.to_string()
                                            checked=Signal::derive(move || (min_rating.get() - value).abs() < 0.01)
                                            on_pick=Callback::new(move |_| { min_rating.set(value); page.set(0); })
                                        />
                                    })
                                    .collect_view()}
                            </div>
                        </FilterCard>

                        <FilterCard title="Amenities" icon="sliders">
                            <div class="flex max-h-56 flex-col gap-0.5 overflow-y-auto pr-1">
                                {all_amenities().into_iter().map(|a| {
                                    let a_owned = a.to_string();
                                    let a_check = a.to_string();
                                    view! {
                                        <label class="flex cursor-pointer items-center gap-2.5 rounded-lg px-2 py-1.5 text-sm text-slate-600 transition-colors hover:bg-slate-50">
                                            <input
                                                type="checkbox"
                                                class="h-4 w-4 rounded border-slate-300 accent-blue-700"
                                                prop:checked=move || amenities.get().contains(&a_check)
                                                on:change=move |_| toggle_amenity(a_owned.clone())
                                            />
                                            <Icon name=tag_icon(a) class="h-3.5 w-3.5 shrink-0 text-slate-400" />
                                            <span class="truncate">{a}</span>
                                        </label>
                                    }
                                }).collect_view()}
                            </div>
                        </FilterCard>

                        <label class="flex cursor-pointer items-center justify-between gap-3 rounded-xl border border-slate-200 bg-white p-3.5 transition-colors hover:border-blue-200">
                            <span class="flex items-center gap-2 text-sm font-semibold text-slate-700">
                                <Icon name="percent" class="h-4 w-4 text-red-600" />
                                "Deals only"
                            </span>
                            <input
                                type="checkbox"
                                class="h-4 w-4 rounded border-slate-300 accent-blue-700"
                                prop:checked=deals_only
                                on:change:target=move |ev| { deals_only.set(ev.target().checked()); page.set(0); }
                            />
                        </label>

                        <Show when=move || (active_count() > 0)>
                            <button
                                on:click=clear_all
                                class="flex items-center justify-center gap-1.5 rounded-xl border border-slate-300 py-2.5 text-sm font-semibold text-slate-600 transition-colors hover:border-red-300 hover:bg-red-50 hover:text-red-600"
                            >
                                <Icon name="x-circle" class="h-4 w-4" />
                                {move || format!("Clear {} filters", active_count())}
                            </button>
                        </Show>

                        <button
                            class="rounded-xl bg-blue-700 py-3 text-sm font-bold text-white lg:hidden"
                            on:click=move |_| filters_open.set(false)
                        >
                            {move || format!("Show {} hotels", results.get().len())}
                        </button>
                    </div>
                </div>
            </aside>

            // ================= RESULTS =================
            <div>
                <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
                    <div class="flex items-center gap-2">
                        <button
                            class="relative flex items-center gap-1.5 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-semibold text-slate-700 transition-colors hover:bg-slate-50 lg:hidden"
                            on:click=move |_| filters_open.set(true)
                        >
                            <Icon name="sliders" class="h-4 w-4" />
                            "Filters"
                            <Show when=move || (active_count() > 0)>
                                <span class="ml-0.5 flex h-5 min-w-5 items-center justify-center rounded-full bg-blue-700 px-1.5 text-[11px] font-bold text-white">
                                    {active_count}
                                </span>
                            </Show>
                        </button>

                        <div class="relative">
                            <button
                                on:click=move |_| sort_open.update(|v| *v = !*v)
                                class="flex items-center gap-1.5 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-semibold text-slate-700 transition-colors hover:bg-slate-50"
                            >
                                <Icon name="sort" class="h-4 w-4" />
                                <span class="hidden sm:inline">{move || sort.get().label()}</span>
                                <span class="sm:hidden">"Sort"</span>
                                <Icon name="chevron-down" class="h-3 w-3" />
                            </button>
                            <Show when=move || sort_open.get()>
                                <div class="absolute left-0 top-full z-30 mt-1 w-56 animate-fade-down overflow-hidden rounded-xl border border-slate-200 bg-white p-1 shadow-xl">
                                    {Sort::ALL.into_iter().map(|s| view! {
                                        <button
                                            on:click=move |_| { sort.set(s); sort_open.set(false); page.set(0); }
                                            class=move || format!(
                                                "flex w-full items-center justify-between rounded-lg px-3 py-2 text-left text-sm transition-colors hover:bg-slate-50 {}",
                                                if sort.get() == s { "font-bold text-blue-700" } else { "text-slate-600" }
                                            )
                                        >
                                            {s.label()}
                                            <Show when=move || sort.get() == s>
                                                <Icon name="check" class="h-3.5 w-3.5" />
                                            </Show>
                                        </button>
                                    }).collect_view()}
                                </div>
                            </Show>
                        </div>
                    </div>

                    <div class="flex items-center gap-3">
                        <span class="hidden text-sm text-slate-500 sm:inline">
                            {move || {
                                let total = results.get().len();
                                let start = if total == 0 { 0 } else { page.get() * PER_PAGE + 1 };
                                let end = ((page.get() + 1) * PER_PAGE).min(total);
                                format!("{start}–{end} of {total}")
                            }}
                        </span>
                        <div class="flex overflow-hidden rounded-lg border border-slate-300">
                            <button
                                aria-label="List view"
                                on:click=move |_| grid_view.set(false)
                                class=move || format!(
                                    "flex h-9 w-9 items-center justify-center transition-colors {}",
                                    if grid_view.get() { "bg-white text-slate-500 hover:bg-slate-50" } else { "bg-blue-700 text-white" }
                                )
                            >
                                <Icon name="list" class="h-4 w-4" />
                            </button>
                            <button
                                aria-label="Grid view"
                                on:click=move |_| grid_view.set(true)
                                class=move || format!(
                                    "flex h-9 w-9 items-center justify-center border-l border-slate-300 transition-colors {}",
                                    if grid_view.get() { "bg-blue-700 text-white" } else { "bg-white text-slate-500 hover:bg-slate-50" }
                                )
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
                            <Chip label=Signal::derive(move || format!("City: {}", city.get())) on_clear=Callback::new(move |_| city.set(String::new())) />
                        </Show>
                        <Show when=move || !text.get().is_empty()>
                            <Chip label=Signal::derive(move || format!("\"{}\"", text.get())) on_clear=Callback::new(move |_| text.set(String::new())) />
                        </Show>
                        <Show when=move || (max_price.get() < PRICE_MAX)>
                            <Chip label=Signal::derive(move || format!("Under ETB {}", thousands(max_price.get()))) on_clear=Callback::new(move |_| max_price.set(PRICE_MAX)) />
                        </Show>
                        <Show when=move || (min_rating.get() > 0.0)>
                            <Chip label=Signal::derive(move || format!("{:.1}+ rating", min_rating.get())) on_clear=Callback::new(move |_| min_rating.set(0.0)) />
                        </Show>
                        <Show when=move || deals_only.get()>
                            <Chip label=Signal::derive(move || "Deals only".to_string()) on_clear=Callback::new(move |_| deals_only.set(false)) />
                        </Show>
                        {move || star_classes.get().into_iter().map(|s| view! {
                            <Chip
                                label=Signal::derive(move || format!("{s}-star"))
                                on_clear=Callback::new(move |_| toggle_star(s))
                            />
                        }).collect_view()}
                        {move || amenities.get().into_iter().map(|a| {
                            let a2 = a.clone();
                            let a3 = a.clone();
                            view! {
                                <Chip
                                    label=Signal::derive(move || a2.clone())
                                    on_clear=Callback::new(move |_| toggle_amenity(a3.clone()))
                                />
                            }
                        }).collect_view()}
                    </div>
                </Show>

                // ---- Result list --------------------------------------------
                <Show
                    when=move || !visible().is_empty()
                    fallback=move || view! {
                        <EmptyState
                            icon="search"
                            title="No hotels match those filters"
                            body="Try widening the price range, removing an amenity, or searching a different city."
                        >
                            <button
                                on:click=clear_all
                                class="rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800"
                            >
                                "Clear all filters"
                            </button>
                        </EmptyState>
                    }
                >
                    <div class=move || if grid_view.get() {
                        "grid gap-5 sm:grid-cols-2 xl:grid-cols-3"
                    } else {
                        "flex flex-col gap-4"
                    }>
                        {move || visible().into_iter().enumerate().map(|(i, h)| {
                            let delay = format!("animation-delay: {}ms", i * 60);
                            view! {
                                <div class="animate-fade-up" style=delay>
                                    <Show
                                        when=move || grid_view.get()
                                        fallback=move || view! { <HotelCard hotel=h /> }
                                    >
                                        <HotelCardCompact hotel=h />
                                    </Show>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </Show>

                // ---- Pagination ---------------------------------------------
                <Show when=move || (page_count() > 1)>
                    <div class="mt-8 flex items-center justify-center gap-1.5 text-sm">
                        <button
                            aria-label="Previous page"
                            class="flex h-9 w-9 items-center justify-center rounded-lg border border-slate-300 text-slate-600 transition-colors hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-40"
                            disabled=move || (page.get() == 0)
                            on:click=move |_| page.update(|p| *p = p.saturating_sub(1))
                        >
                            <Icon name="chevron-left" class="h-4 w-4" />
                        </button>

                        {move || (0..page_count()).map(|i| view! {
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
                                {i + 1}
                            </button>
                        }).collect_view()}

                        <button
                            aria-label="Next page"
                            class="flex h-9 w-9 items-center justify-center rounded-lg border border-slate-300 text-slate-600 transition-colors hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-40"
                            disabled=move || (page.get() + 1 >= page_count())
                            on:click=move |_| page.update(|p| *p += 1)
                        >
                            <Icon name="chevron-right" class="h-4 w-4" />
                        </button>
                    </div>
                </Show>

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
