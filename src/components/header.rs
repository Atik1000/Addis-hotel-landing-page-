//! Site header.
//!
//! The menu is deliberately three items — Home, Popular Cities, Hotels — with
//! the destinations sitting behind a dropdown fed by
//! `GET /organizations/public/cities/`, so the list always matches the cities
//! that actually have hotels rather than a hardcoded set.

use crate::api::list_cities;
use crate::components::{pluralize, Icon};
use crate::images::city_image;
use crate::session;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

/// Booking links, drawer only — they are utilities rather than menu items.
struct NavLink {
    label: &'static str,
    href: &'static str,
    icon: &'static str,
}

const DRAWER_EXTRA: &[NavLink] = &[
    NavLink { label: "My Reservation", href: "/my-reservations", icon: "calendar" },
    NavLink { label: "Retrieve Booking", href: "/retrieve-booking", icon: "search" },
    NavLink { label: "My Reviews", href: "/my-reviews", icon: "star" },
];

fn city_href(city: &str) -> String {
    format!("/hotels?city={}", city.replace(' ', "+"))
}

#[component]
pub fn Header() -> impl IntoView {
    let menu_open = RwSignal::new(false);
    let cities_open = RwSignal::new(false);
    let drawer_cities_open = RwSignal::new(true);
    let lang_open = RwSignal::new(false);
    let language = RwSignal::new("EN");
    let location = use_location();
    let guest = session::use_guest();

    // Shared by the desktop dropdown and the drawer's expandable section.
    let cities = Resource::new(|| (), |_| async move { list_cities(Some(12)).await });

    let is_active = move |href: &'static str| {
        let path = location.pathname.get();
        if href == "/" {
            path == "/"
        } else {
            path.starts_with(href)
        }
    };
    // "Popular Cities" reads as active whenever a city filter is applied.
    let cities_active = move || location.search.get().contains("city=");

    // Any navigation should dismiss the overlays.
    Effect::new(move |_| {
        let _ = location.pathname.get();
        let _ = location.search.get();
        menu_open.set(false);
        cities_open.set(false);
        lang_open.set(false);
    });

    let nav_class = move |active: bool| {
        format!(
            "link-underline text-sm font-semibold transition-colors duration-200 {}",
            if active { "text-blue-700" } else { "text-slate-600 hover:text-blue-700" }
        )
    };

    view! {
        <header class="sticky top-0 z-40 border-b border-slate-200/80 glass">
            <div class="mx-auto flex max-w-6xl items-center justify-between gap-4 px-4 py-3">
                <A href="/" attr:class="group flex shrink-0 items-center gap-2.5">
                    <span class="relative flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-blue-600 to-indigo-700 text-white shadow-md shadow-blue-700/25 transition-transform duration-300 group-hover:scale-105 group-hover:rotate-3">
                        <Icon name="building" class="h-5 w-5" />
                    </span>
                    <span class="leading-tight">
                        <span class="block text-base font-bold tracking-tight text-ink">"Horn of Africa"</span>
                        <span class="block text-[10px] font-semibold tracking-[0.16em] text-blue-700">"HOTEL PORTAL"</span>
                    </span>
                </A>

                // ---- Desktop menu: Home · Popular Cities · Hotels ----------
                <nav class="hidden items-center gap-7 lg:flex">
                    <A href="/" attr:class=move || nav_class(is_active("/"))>"Home"</A>

                    <div
                        class="relative"
                        on:mouseenter=move |_| cities_open.set(true)
                        on:mouseleave=move |_| cities_open.set(false)
                    >
                        <button
                            aria-haspopup="true"
                            aria-expanded=move || if cities_open.get() { "true" } else { "false" }
                            on:click=move |_| cities_open.update(|v| *v = !*v)
                            class=move || format!("flex items-center gap-1 {}", nav_class(cities_active()))
                        >
                            "Popular Cities"
                            <span class=move || format!(
                                "inline-flex transition-transform duration-200 {}",
                                if cities_open.get() { "rotate-180" } else { "" }
                            )>
                                <Icon name="chevron-down" class="h-3.5 w-3.5" />
                            </span>
                        </button>

                        <Show when=move || cities_open.get()>
                            // Bridges the gap so the panel survives the cursor
                            // crossing from the trigger.
                            <div class="absolute left-1/2 top-full z-40 w-[30rem] -translate-x-1/2 pt-3">
                                <div class="animate-fade-down overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-xl shadow-slate-900/10">
                                    <Suspense fallback=|| view! {
                                        <div class="grid grid-cols-2 gap-2 p-3">
                                            {(0..4).map(|_| view! { <div class="skeleton h-14 rounded-xl"></div> }).collect_view()}
                                        </div>
                                    }>
                                        {move || Suspend::new(async move {
                                            let list = cities.await.map(|p| p.items).unwrap_or_default();
                                            if list.is_empty() {
                                                return view! {
                                                    <p class="px-4 py-6 text-center text-sm text-slate-500">
                                                        "No destinations are listed yet."
                                                    </p>
                                                }.into_any();
                                            }
                                            view! {
                                                <div class="grid grid-cols-2 gap-1.5 p-3">
                                                    {list.into_iter().map(|c| {
                                                        let img = city_image(&c.city, c.featured_image.as_deref(), 160);
                                                        let count = c.hotel_count;
                                                        view! {
                                                            <a
                                                                href=city_href(&c.city)
                                                                class="group flex items-center gap-3 rounded-xl p-2 transition-colors hover:bg-slate-50"
                                                            >
                                                                <img
                                                                    src=img
                                                                    alt=""
                                                                    loading="lazy"
                                                                    class="h-11 w-11 shrink-0 rounded-lg object-cover"
                                                                />
                                                                <span class="min-w-0">
                                                                    <span class="block truncate text-sm font-semibold text-ink group-hover:text-blue-700">
                                                                        {c.city.clone()}
                                                                    </span>
                                                                    <span class="block truncate text-[11px] text-slate-500">
                                                                        {pluralize(count, "hotel")}
                                                                        {c.country.clone().map(|co| format!(" · {co}")).unwrap_or_default()}
                                                                    </span>
                                                                </span>
                                                            </a>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            }.into_any()
                                        })}
                                    </Suspense>
                                    <A
                                        href="/hotels"
                                        attr:class="flex items-center justify-center gap-1.5 border-t border-slate-100 bg-slate-50/70 px-4 py-2.5 text-xs font-bold text-blue-700 transition-colors hover:bg-slate-100"
                                    >
                                        "View all destinations"
                                        <Icon name="arrow-right" class="h-3.5 w-3.5" />
                                    </A>
                                </div>
                            </div>
                        </Show>
                    </div>

                    <A href="/hotels" attr:class=move || nav_class(is_active("/hotels"))>"Hotels"</A>
                </nav>

                <div class="flex items-center gap-2">
                    <div class="relative hidden sm:block">
                        <button
                            on:click=move |_| lang_open.update(|v| *v = !*v)
                            class="flex items-center gap-1.5 rounded-lg px-2.5 py-2 text-sm font-semibold text-slate-600 transition-colors hover:bg-slate-100 hover:text-ink"
                        >
                            <Icon name="globe" class="h-4 w-4" />
                            {move || language.get()}
                            <Icon name="chevron-down" class="h-3.5 w-3.5 transition-transform duration-200" />
                        </button>
                        <Show when=move || lang_open.get()>
                            <div class="absolute right-0 top-full z-40 mt-1 w-40 animate-fade-down overflow-hidden rounded-xl border border-slate-200 bg-white p-1 shadow-xl shadow-slate-900/10">
                                {["EN", "አማርኛ", "Soomaali", "العربية"].iter().map(|l| {
                                    let l = *l;
                                    view! {
                                        <button
                                            on:click=move |_| { language.set(l); lang_open.set(false); }
                                            class=move || format!(
                                                "flex w-full items-center justify-between rounded-lg px-3 py-2 text-left text-sm transition-colors hover:bg-slate-50 {}",
                                                if language.get() == l { "font-semibold text-blue-700" } else { "text-slate-600" }
                                            )
                                        >
                                            {l}
                                            <Show when=move || language.get() == l>
                                                <Icon name="check" class="h-3.5 w-3.5" />
                                            </Show>
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </Show>
                    </div>

                    <A
                        href="/my-reservations"
                        attr:class="hidden items-center gap-1.5 rounded-lg border border-slate-300 px-3 py-2 text-sm font-semibold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-sm md:flex"
                    >
                        <Icon name="ticket" class="h-4 w-4" />
                        "My Reservation"
                    </A>

                    // Signed in: straight to the guest's own reviews. Signed
                    // out: nothing, since booking needs no account and an
                    // unexplained "Sign in" invites the wrong expectation.
                    <Show when=move || guest.get().is_some()>
                        <A
                            href="/my-reviews"
                            attr:class="hidden items-center gap-1.5 rounded-lg border border-blue-200 bg-blue-50 px-3 py-2 text-sm font-semibold text-blue-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 md:flex"
                        >
                            <Icon name="user-check" class="h-4 w-4" />
                            {move || guest.get().map(|g| g.short_name()).unwrap_or_default()}
                        </A>
                    </Show>

                    <A
                        href="/hotels"
                        attr:class="sheen hidden items-center gap-1.5 rounded-lg bg-blue-700 px-4 py-2 text-sm font-semibold text-white shadow-md shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-lg active:scale-[0.98] sm:flex"
                    >
                        <Icon name="search" class="h-4 w-4" />
                        "Find a Hotel"
                    </A>

                    <button
                        aria-label="Open menu"
                        class="flex h-10 w-10 items-center justify-center rounded-lg text-slate-700 transition-colors hover:bg-slate-100 lg:hidden"
                        on:click=move |_| menu_open.set(true)
                    >
                        <Icon name="menu" class="h-5 w-5" />
                    </button>
                </div>
            </div>

            // ---- Mobile / tablet drawer -------------------------------------
            <Show when=move || menu_open.get()>
                <div class="fixed inset-0 z-50 lg:hidden">
                    <div
                        class="absolute inset-0 animate-fade-in bg-slate-900/50 backdrop-blur-sm"
                        on:click=move |_| menu_open.set(false)
                    ></div>

                    <aside class="absolute right-0 top-0 flex h-full w-[86%] max-w-sm animate-drawer-in flex-col bg-white shadow-2xl">
                        <div class="flex items-center justify-between border-b border-slate-200 px-5 py-4">
                            <span class="flex items-center gap-2">
                                <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-blue-600 to-indigo-700 text-white">
                                    <Icon name="building" class="h-4 w-4" />
                                </span>
                                <span class="text-sm font-bold text-ink">"Horn of Africa"</span>
                            </span>
                            <button
                                aria-label="Close menu"
                                class="flex h-9 w-9 items-center justify-center rounded-lg text-slate-500 transition-colors hover:bg-slate-100 hover:text-ink"
                                on:click=move |_| menu_open.set(false)
                            >
                                <Icon name="x" class="h-5 w-5" />
                            </button>
                        </div>

                        <nav class="flex-1 overflow-y-auto p-4">
                            <p class="px-3 pb-2 text-[11px] font-semibold uppercase tracking-wider text-slate-400">"Menu"</p>

                            <A
                                href="/"
                                attr:class=move || format!(
                                    "mb-1 flex items-center gap-3 rounded-xl px-3 py-3 text-sm font-semibold transition-colors {}",
                                    if is_active("/") { "bg-blue-50 text-blue-700" } else { "text-slate-700 hover:bg-slate-50" }
                                )
                            >
                                <Icon name="home" class="h-4.5 w-4.5" />
                                "Home"
                                <Icon name="chevron-right" class="ml-auto h-4 w-4 text-slate-300" />
                            </A>

                            <button
                                on:click=move |_| drawer_cities_open.update(|v| *v = !*v)
                                class="mb-1 flex w-full items-center gap-3 rounded-xl px-3 py-3 text-sm font-semibold text-slate-700 transition-colors hover:bg-slate-50"
                            >
                                <Icon name="map-pin" class="h-4.5 w-4.5" />
                                "Popular Cities"
                                <span class=move || format!(
                                    "ml-auto inline-flex text-slate-400 transition-transform duration-200 {}",
                                    if drawer_cities_open.get() { "rotate-180" } else { "" }
                                )>
                                    <Icon name="chevron-down" class="h-4 w-4" />
                                </span>
                            </button>

                            <Show when=move || drawer_cities_open.get()>
                                <div class="mb-2 ml-4 border-l border-slate-200 pl-3">
                                    <Suspense fallback=|| view! {
                                        <div class="flex flex-col gap-1.5 py-1">
                                            {(0..3).map(|_| view! { <div class="skeleton h-9 rounded-lg"></div> }).collect_view()}
                                        </div>
                                    }>
                                        {move || Suspend::new(async move {
                                            let list = cities.await.map(|p| p.items).unwrap_or_default();
                                            if list.is_empty() {
                                                return view! {
                                                    <p class="px-3 py-2 text-xs text-slate-500">"No destinations yet."</p>
                                                }.into_any();
                                            }
                                            view! {
                                                <>
                                                    {list.into_iter().map(|c| {
                                                        let count = c.hotel_count;
                                                        view! {
                                                            <a
                                                                href=city_href(&c.city)
                                                                class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-slate-600 transition-colors hover:bg-slate-50 hover:text-blue-700"
                                                            >
                                                                {c.city.clone()}
                                                                <span class="ml-auto text-[11px] text-slate-400">{count}</span>
                                                            </a>
                                                        }
                                                    }).collect_view()}
                                                </>
                                            }.into_any()
                                        })}
                                    </Suspense>
                                </div>
                            </Show>

                            <A
                                href="/hotels"
                                attr:class=move || format!(
                                    "mb-1 flex items-center gap-3 rounded-xl px-3 py-3 text-sm font-semibold transition-colors {}",
                                    if is_active("/hotels") { "bg-blue-50 text-blue-700" } else { "text-slate-700 hover:bg-slate-50" }
                                )
                            >
                                <Icon name="building" class="h-4.5 w-4.5" />
                                "Hotels"
                                <Icon name="chevron-right" class="ml-auto h-4 w-4 text-slate-300" />
                            </A>

                            <p class="mt-5 px-3 pb-2 text-[11px] font-semibold uppercase tracking-wider text-slate-400">"Your booking"</p>
                            {DRAWER_EXTRA.iter().enumerate().map(|(i, link)| {
                                let delay = format!("animation-delay: {}ms", 240 + i * 40);
                                view! {
                                    <A
                                        href=link.href
                                        attr:style=delay
                                        attr:class=move || format!(
                                            "mb-1 flex animate-slide-in-right items-center gap-3 rounded-xl px-3 py-3 text-sm font-semibold transition-colors {}",
                                            if is_active(link.href) { "bg-blue-50 text-blue-700" } else { "text-slate-700 hover:bg-slate-50" }
                                        )
                                    >
                                        <Icon name=link.icon class="h-4.5 w-4.5" />
                                        {link.label}
                                        <Icon name="chevron-right" class="ml-auto h-4 w-4 text-slate-300" />
                                    </A>
                                }
                            }).collect_view()}
                        </nav>

                        <div class="border-t border-slate-200 p-4">
                            <A
                                href="/hotels"
                                attr:class="sheen flex w-full items-center justify-center gap-2 rounded-xl bg-blue-700 py-3 text-sm font-semibold text-white shadow-md shadow-blue-700/25 transition-colors hover:bg-blue-800"
                            >
                                <Icon name="search" class="h-4 w-4" />
                                "Find a Hotel"
                            </A>
                            <div class="mt-3 flex items-center justify-center gap-2 text-xs text-slate-400">
                                <Icon name="shield-check" class="h-3.5 w-3.5 text-blue-600" />
                                "No booking fees · Pay at the hotel"
                            </div>
                        </div>
                    </aside>
                </div>
            </Show>
        </header>
    }
}
