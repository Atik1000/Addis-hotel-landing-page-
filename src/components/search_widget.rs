//! The destination / dates / guests search panel.
//!
//! Submitting navigates to `/hotels` with the criteria as query parameters so
//! the listings page can pick them up (and so a search is linkable).

use crate::components::Icon;
use crate::api::list_cities;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// Percent-encodes the handful of characters that matter for a query value.
fn encode(value: &str) -> String {
    value
        .chars()
        .map(|c| match c {
            ' ' => "+".to_string(),
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            other => format!("%{:02X}", other as u32 & 0xFF),
        })
        .collect()
}

#[component]
pub fn SearchWidget(
    /// `true` renders the compact single-row layout used on the listings page.
    #[prop(default = false)]
    horizontal: bool,
    #[prop(default = "")] initial_destination: &'static str,
) -> impl IntoView {
    let navigate = use_navigate();
    let destination = RwSignal::new(initial_destination.to_string());
    // Seeded from tomorrow so the dates are always bookable; the API rejects a
    // check-in in the past.
    let check_in = RwSignal::new(default_date(1));
    let check_out = RwSignal::new(default_date(3));
    let guests = RwSignal::new(2u32);
    let rooms = RwSignal::new(1u32);
    let suggestions_open = RwSignal::new(false);
    let guests_open = RwSignal::new(false);

    let cities = Resource::new(|| (), |_| async move { list_cities(Some(100)).await });
    let matches = move || {
        let q = destination.get().to_lowercase();
        cities
            .get()
            .and_then(Result::ok)
            .map(|p| p.items)
            .unwrap_or_default()
            .into_iter()
            .filter(|c| {
                q.is_empty()
                    || c.city.to_lowercase().contains(&q)
                    || c.country.as_deref().unwrap_or("").to_lowercase().contains(&q)
            })
            .take(8)
            .collect::<Vec<_>>()
    };

    let submit = {
        let navigate = navigate.clone();
        move |_| {
            let query = format!(
                "/hotels?city={}&check_in={}&check_out={}&guests={}&rooms={}",
                encode(&destination.get()),
                encode(&check_in.get()),
                encode(&check_out.get()),
                guests.get(),
                rooms.get(),
            );
            suggestions_open.set(false);
            guests_open.set(false);
            navigate(&query, Default::default());
        }
    };

    let field_class = "w-full rounded-xl border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-800 transition-all duration-200 focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100";

    view! {
        <div class=if horizontal {
            "rounded-2xl border border-slate-200 bg-white p-3 shadow-lg shadow-slate-900/5"
        } else {
            "rounded-2xl bg-white p-5 shadow-2xl shadow-slate-950/25 ring-1 ring-white/40"
        }>
            <Show when=move || !horizontal>
                <h2 class="mb-3 flex items-center gap-2 text-sm font-bold text-slate-800">
                    <Icon name="search" class="h-4 w-4 text-blue-700" />
                    "Where are you going?"
                </h2>
            </Show>

            <div class=if horizontal {
                "grid gap-2 lg:grid-cols-[1.4fr_1fr_1fr_1fr_auto]"
            } else {
                "grid gap-3"
            }>
                // ---- Destination with typeahead ----------------------------
                <div class="relative">
                    <Show when=move || !horizontal>
                        <label class="mb-1 block text-xs font-semibold text-slate-500">"Destination"</label>
                    </Show>
                    <div class="relative">
                        <Icon name="map-pin" class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
                        <input
                            type="text"
                            placeholder="City, hotel or area"
                            class=format!("{field_class} pl-9")
                            prop:value=destination
                            on:focus=move |_| suggestions_open.set(true)
                            on:input:target=move |ev| {
                                destination.set(ev.target().value());
                                suggestions_open.set(true);
                            }
                        />
                        <Show when=move || !destination.get().is_empty()>
                            <button
                                aria-label="Clear destination"
                                class="absolute right-2.5 top-1/2 -translate-y-1/2 rounded p-1 text-slate-400 transition-colors hover:text-slate-700"
                                on:click=move |_| destination.set(String::new())
                            >
                                <Icon name="x" class="h-3.5 w-3.5" />
                            </button>
                        </Show>
                    </div>

                    <Show when=move || suggestions_open.get() && !matches().is_empty()>
                        <div class="absolute left-0 right-0 top-full z-30 mt-1.5 max-h-72 animate-fade-down overflow-y-auto rounded-xl border border-slate-200 bg-white p-1.5 shadow-2xl shadow-slate-900/15">
                            <p class="px-2.5 py-1.5 text-[11px] font-bold uppercase tracking-wider text-slate-400">"Popular destinations"</p>
                            {move || matches().into_iter().map(|c| {
                                let name = c.city.clone();
                                let thumb = c.featured_image.clone().filter(|u| u.starts_with("http"));
                                view! {
                                    <button
                                        class="flex w-full items-center gap-3 rounded-lg px-2.5 py-2 text-left transition-colors hover:bg-blue-50"
                                        on:click=move |_| {
                                            destination.set(name.clone());
                                            suggestions_open.set(false);
                                        }
                                    >
                                        {match thumb {
                                            Some(src) => view! {
                                                <img src=src alt="" class="h-9 w-9 shrink-0 rounded-lg object-cover" />
                                            }.into_any(),
                                            None => view! {
                                                <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-700">
                                                    <Icon name="map-pin" class="h-4 w-4" />
                                                </span>
                                            }.into_any(),
                                        }}
                                        <span class="min-w-0 flex-1">
                                            <span class="block truncate text-sm font-semibold text-slate-800">{c.city.clone()}</span>
                                            <span class="block truncate text-xs text-slate-400">
                                                {c.country.clone().unwrap_or_default()}
                                            </span>
                                        </span>
                                        <span class="shrink-0 text-xs font-medium text-slate-400">
                                            {format!("{} hotels", c.hotel_count)}
                                        </span>
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    </Show>
                </div>

                // ---- Dates --------------------------------------------------
                <div class=if horizontal { "" } else { "grid grid-cols-2 gap-3" }>
                    <div>
                        <Show when=move || !horizontal>
                            <label class="mb-1 block text-xs font-semibold text-slate-500">"Check-in"</label>
                        </Show>
                        <input
                            type="date"
                            class=field_class
                            prop:value=check_in
                            on:input:target=move |ev| check_in.set(ev.target().value())
                        />
                    </div>
                    <Show when=move || !horizontal>
                        <div>
                            <label class="mb-1 block text-xs font-semibold text-slate-500">"Check-out"</label>
                            <input
                                type="date"
                                class=field_class
                                prop:value=check_out
                                on:input:target=move |ev| check_out.set(ev.target().value())
                            />
                        </div>
                    </Show>
                </div>

                <Show when=move || horizontal>
                    <div>
                        <input
                            type="date"
                            class=field_class
                            prop:value=check_out
                            on:input:target=move |ev| check_out.set(ev.target().value())
                        />
                    </div>
                </Show>

                // ---- Guests & rooms stepper --------------------------------
                <div class="relative">
                    <Show when=move || !horizontal>
                        <label class="mb-1 block text-xs font-semibold text-slate-500">"Guests & Rooms"</label>
                    </Show>
                    <button
                        on:click=move |_| guests_open.update(|v| *v = !*v)
                        class=format!("{field_class} flex items-center justify-between gap-2 text-left")
                    >
                        <span class="flex items-center gap-2 truncate">
                            <Icon name="users" class="h-4 w-4 shrink-0 text-slate-400" />
                            {move || format!(
                                "{} {}, {} {}",
                                guests.get(),
                                if guests.get() == 1 { "Guest" } else { "Guests" },
                                rooms.get(),
                                if rooms.get() == 1 { "Room" } else { "Rooms" },
                            )}
                        </span>
                        <Icon name="chevron-down" class="h-3.5 w-3.5 shrink-0 text-slate-400" />
                    </button>

                    <Show when=move || guests_open.get()>
                        <div class="absolute left-0 right-0 top-full z-30 mt-1.5 animate-fade-down rounded-xl border border-slate-200 bg-white p-3 shadow-2xl shadow-slate-900/15">
                            <Stepper label="Guests" hint="Ages 13 or above" value=guests min=1 max=16 />
                            <div class="my-2 h-px bg-slate-100"></div>
                            <Stepper label="Rooms" hint="Separate rooms" value=rooms min=1 max=8 />
                            <button
                                class="mt-3 w-full rounded-lg bg-slate-900 py-2 text-sm font-semibold text-white transition-colors hover:bg-slate-800"
                                on:click=move |_| guests_open.set(false)
                            >
                                "Done"
                            </button>
                        </div>
                    </Show>
                </div>

                // ---- Submit -------------------------------------------------
                <button
                    on:click=submit
                    class=format!(
                        "sheen flex items-center justify-center gap-2 rounded-xl bg-blue-700 font-semibold text-white shadow-lg shadow-blue-700/30 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-xl active:scale-[0.98] {}",
                        if horizontal { "px-6 py-2.5 text-sm" } else { "mt-1 w-full py-3" }
                    )
                >
                    <Icon name="search" class="h-4 w-4" />
                    {if horizontal { "Search" } else { "Search Hotels" }}
                </button>
            </div>
        </div>
    }
}

#[component]
fn Stepper(
    label: &'static str,
    hint: &'static str,
    value: RwSignal<u32>,
    min: u32,
    max: u32,
) -> impl IntoView {
    let btn = "flex h-8 w-8 items-center justify-center rounded-lg border border-slate-300 text-slate-600 transition-all duration-150 hover:border-blue-400 hover:text-blue-700 active:scale-90 disabled:cursor-not-allowed disabled:opacity-35 disabled:hover:border-slate-300 disabled:hover:text-slate-600";
    view! {
        <div class="flex items-center justify-between gap-4 py-1">
            <span>
                <span class="block text-sm font-semibold text-slate-800">{label}</span>
                <span class="block text-xs text-slate-400">{hint}</span>
            </span>
            <span class="flex items-center gap-2.5">
                <button
                    class=btn
                    disabled=move || value.get() <= min
                    on:click=move |_| value.update(|v| *v = v.saturating_sub(1).max(min))
                >
                    <Icon name="minus" class="h-3.5 w-3.5" />
                </button>
                <span class="w-6 text-center text-sm font-bold tabular-nums text-slate-900">{move || value.get()}</span>
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

/// An ISO date `offset` days from today. Empty on the server, where there is no
/// clock the guest would recognise — hydration fills it in.
fn default_date(offset: i64) -> String {
    #[cfg(feature = "hydrate")]
    {
        let now = js_sys::Date::new_0();
        let shifted = js_sys::Date::new(&js_sys::Date::new_0().into());
        shifted.set_date(now.get_date() + offset as u32);
        return format!(
            "{:04}-{:02}-{:02}",
            shifted.get_full_year(),
            shifted.get_month() + 1,
            shifted.get_date(),
        );
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = offset;
        String::new()
    }
}
