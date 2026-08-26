use crate::components::Icon;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

struct NavLink {
    label: &'static str,
    href: &'static str,
    icon: &'static str,
}

const PRIMARY_NAV: &[NavLink] = &[
    NavLink { label: "Home", href: "/", icon: "home" },
    NavLink { label: "Hotels", href: "/hotels", icon: "building" },
    NavLink { label: "How it works", href: "/how-it-works", icon: "sparkles" },
    NavLink { label: "About", href: "/about", icon: "info" },
    NavLink { label: "Help", href: "/faq", icon: "help-circle" },
];

const DRAWER_EXTRA: &[NavLink] = &[
    NavLink { label: "My Reservation", href: "/my-reservations", icon: "calendar" },
    NavLink { label: "Retrieve Booking", href: "/retrieve-booking", icon: "search" },
    NavLink { label: "Contact Us", href: "/contact", icon: "headset" },
];

#[component]
pub fn Header() -> impl IntoView {
    let menu_open = RwSignal::new(false);
    let lang_open = RwSignal::new(false);
    let language = RwSignal::new("EN");
    let location = use_location();

    let is_active = move |href: &'static str| {
        let path = location.pathname.get();
        if href == "/" {
            path == "/"
        } else {
            path.starts_with(href)
        }
    };

    // Any navigation should dismiss the overlays.
    Effect::new(move |_| {
        let _ = location.pathname.get();
        menu_open.set(false);
        lang_open.set(false);
    });

    view! {
        <header class="sticky top-0 z-40 border-b border-slate-200/80 glass">
            <div class="mx-auto flex max-w-6xl items-center justify-between gap-4 px-4 py-3">
                <A href="/" attr:class="group flex shrink-0 items-center gap-2.5">
                    <span class="relative flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-blue-600 to-indigo-700 text-white shadow-md shadow-blue-700/25 transition-transform duration-300 group-hover:scale-105 group-hover:rotate-3">
                        <Icon name="building" class="h-5 w-5" />
                    </span>
                    <span class="leading-tight">
                        <span class="block text-base font-extrabold tracking-tight text-slate-900">"Horn of Africa"</span>
                        <span class="block text-[10px] font-semibold tracking-[0.16em] text-blue-700">"HOTEL PORTAL"</span>
                    </span>
                </A>

                <nav class="hidden items-center gap-7 lg:flex">
                    {PRIMARY_NAV.iter().map(|link| view! {
                        <A
                            href=link.href
                            attr:data-active=move || if is_active(link.href) { "true" } else { "false" }
                            attr:class=move || format!(
                                "link-underline text-sm font-semibold transition-colors duration-200 {}",
                                if is_active(link.href) { "text-blue-700" } else { "text-slate-600 hover:text-blue-700" }
                            )
                        >
                            {link.label}
                        </A>
                    }).collect_view()}
                </nav>

                <div class="flex items-center gap-2">
                    <div class="relative hidden sm:block">
                        <button
                            on:click=move |_| lang_open.update(|v| *v = !*v)
                            class="flex items-center gap-1.5 rounded-lg px-2.5 py-2 text-sm font-semibold text-slate-600 transition-colors hover:bg-slate-100 hover:text-slate-900"
                        >
                            <Icon name="globe" class="h-4 w-4" />
                            {move || language.get()}
                            <Icon
                                name="chevron-down"
                                class="h-3.5 w-3.5 transition-transform duration-200"
                            />
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
                                <span class="text-sm font-extrabold text-slate-900">"Horn of Africa"</span>
                            </span>
                            <button
                                aria-label="Close menu"
                                class="flex h-9 w-9 items-center justify-center rounded-lg text-slate-500 transition-colors hover:bg-slate-100 hover:text-slate-900"
                                on:click=move |_| menu_open.set(false)
                            >
                                <Icon name="x" class="h-5 w-5" />
                            </button>
                        </div>

                        <nav class="flex-1 overflow-y-auto p-4">
                            <p class="px-3 pb-2 text-[11px] font-semibold uppercase tracking-wider text-slate-400">"Explore"</p>
                            {PRIMARY_NAV.iter().enumerate().map(|(i, link)| {
                                let delay = format!("animation-delay: {}ms", 40 + i * 40);
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
