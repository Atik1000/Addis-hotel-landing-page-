use crate::components::{provide_toasts, Footer, Header, MobileTabBar, ToastHost};
use crate::pages::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_location,
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="theme-color" content="#1d4ed8"/>
                <meta name="description" content="Discover and reserve verified hotels across Ethiopia, Kenya, Somalia, Somaliland and Djibouti. Pay at the hotel, no booking fees, no account required."/>
                <meta property="og:title" content="Horn of Africa Hotel Portal"/>
                <meta property="og:description" content="Reserve hotels across the Horn of Africa in seconds. Pay at the hotel, no booking fees."/>
                <meta property="og:type" content="website"/>
                <link rel="preconnect" href="https://images.unsplash.com"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-slate-50 text-slate-800 antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_toasts();

    view! {
        <Stylesheet id="leptos" href="/pkg/addis-landing-website.css"/>
        <Title text="Horn of Africa Hotel Portal"/>

        <Router>
            <ScrollToTopOnNavigate/>
            <div class="flex min-h-screen flex-col">
                <Header/>
                <main class="flex-1">
                    <Routes fallback=|| view! { <NotFoundPage/> }>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/hotels") view=ListingsPage/>
                        <Route path=path!("/hotels/:id") view=HotelDetailsPage/>
                        <Route path=path!("/hotels/:id/reserve/:room_id") view=ReservationFormPage/>
                        <Route path=path!("/confirmation/:booking_ref") view=ConfirmationPage/>
                        <Route path=path!("/my-reservations") view=MyReservationsPage/>
                        <Route path=path!("/reservation/:booking_ref") view=ReservationDetailsPage/>
                        <Route path=path!("/retrieve-booking") view=RetrieveBookingPage/>
                        <Route path=path!("/how-it-works") view=HowItWorksPage/>
                        <Route path=path!("/about") view=AboutPage/>
                        <Route path=path!("/contact") view=ContactPage/>
                        <Route path=path!("/faq") view=FaqPage/>
                        <Route path=path!("/terms") view=TermsPage/>
                        <Route path=path!("/privacy") view=PrivacyPage/>
                    </Routes>
                </main>
                <Footer/>
                // Sits above the tab bar on small screens.
                <div class="h-14 sm:hidden"></div>
                <MobileTabBar/>
                <ToastHost/>
            </div>
        </Router>
    }
}

/// Resets the scroll offset on every client-side route change.
///
/// Without this a link followed from halfway down the listings drops you into
/// the middle of the next page. Must live inside `<Router>` so `use_location`
/// has a router context; effects never run on the server, so this is
/// client-only.
#[component]
fn ScrollToTopOnNavigate() -> impl IntoView {
    let pathname = use_location().pathname;

    Effect::new(move |prev: Option<String>| {
        let path = pathname.get();
        if prev.is_some_and(|p| p != path) {
            // `scroll-behavior: smooth` is set site-wide; a route change should
            // land instantly rather than animate the whole page past the user.
            let opts = web_sys::ScrollToOptions::new();
            opts.set_top(0.0);
            opts.set_left(0.0);
            opts.set_behavior(web_sys::ScrollBehavior::Instant);
            window().scroll_to_with_scroll_to_options(&opts);
        }
        path
    });
}
