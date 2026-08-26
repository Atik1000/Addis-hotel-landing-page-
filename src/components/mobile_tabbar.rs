use crate::components::Icon;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

struct Tab {
    icon: &'static str,
    label: &'static str,
    href: &'static str,
}

const TABS: &[Tab] = &[
    Tab { icon: "home", label: "Home", href: "/" },
    Tab { icon: "search", label: "Search", href: "/hotels" },
    Tab { icon: "ticket", label: "Bookings", href: "/my-reservations" },
    Tab { icon: "help-circle", label: "Help", href: "/faq" },
];

#[component]
pub fn MobileTabBar() -> impl IntoView {
    let location = use_location();
    let is_active = move |href: &'static str| {
        let path = location.pathname.get();
        if href == "/" { path == "/" } else { path.starts_with(href) }
    };

    view! {
        <nav class="fixed inset-x-0 bottom-0 z-30 flex border-t border-slate-200 bg-white/95 pb-[env(safe-area-inset-bottom)] backdrop-blur-lg sm:hidden">
            {TABS.iter().map(|tab| view! {
                <A
                    href=tab.href
                    attr:class=move || format!(
                        "relative flex flex-1 flex-col items-center gap-1 py-2.5 text-[11px] font-medium transition-colors duration-200 {}",
                        if is_active(tab.href) { "text-blue-700" } else { "text-slate-400" }
                    )
                >
                    <span class=move || format!(
                        "absolute inset-x-5 top-0 h-0.5 rounded-b-full bg-blue-700 transition-transform duration-300 {}",
                        if is_active(tab.href) { "scale-x-100" } else { "scale-x-0" }
                    )></span>
                    <Icon
                        name=tab.icon
                        class="h-5 w-5"
                    />
                    <span>{tab.label}</span>
                </A>
            }).collect_view()}
        </nav>
    }
}
