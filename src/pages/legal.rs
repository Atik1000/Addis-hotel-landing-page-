//! Terms and Privacy. Both share the same document shell.

use crate::components::{Breadcrumbs, Icon};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

struct Section {
    heading: &'static str,
    paragraphs: &'static [&'static str],
}

const TERMS: &[Section] = &[
    Section {
        heading: "1. What this service is",
        paragraphs: &[
            "Horn of Africa Hotel Portal lists hotels across Ethiopia, Kenya, Somalia, Somaliland and Djibouti, and lets you reserve a room without creating an account or entering payment details.",
            "We are an introduction service. The accommodation contract is between you and the hotel. We are not the supplier of the room and do not take payment for it.",
        ],
    },
    Section {
        heading: "2. Making a reservation",
        paragraphs: &[
            "A reservation is confirmed the moment you receive a booking reference. The hotel receives the same reservation in its dashboard at that time.",
            "You are responsible for the accuracy of the details you enter. If the name on the reservation does not match the identification presented at check-in, the hotel may refuse the booking.",
            "Rates shown are the hotel's own published rates for the dates selected, quoted per room per night and inclusive of applicable taxes and levies where stated.",
        ],
    },
    Section {
        heading: "3. Payment",
        paragraphs: &[
            "All payment is made directly to the hotel at check-in. We do not collect card details, take deposits, or charge booking fees or commissions to guests.",
            "The hotel charges in its own local currency. Rates displayed in Ethiopian Birr are indicative for comparison; the amount charged is the hotel's local-currency equivalent.",
            "Accepted payment methods vary by property. Most accept cash and major cards; mobile money availability differs.",
        ],
    },
    Section {
        heading: "4. Changes and cancellation",
        paragraphs: &[
            "Rooms marked 'Free cancellation' may be cancelled without charge up to 24 hours before the check-in date. Non-refundable rates are charged in full by the hotel if cancelled or if you do not arrive.",
            "Cancel from My Reservation, from a retrieved booking, or by calling the hotel directly. Date changes are handled by the hotel, not by us.",
            "If a hotel is unable to honour a confirmed reservation, contact us and we will help find comparable accommodation. We are not liable for the hotel's costs or for consequential losses.",
        ],
    },
    Section {
        heading: "5. Your conduct",
        paragraphs: &[
            "Do not make reservations you do not intend to honour, use false details, or use the service to disrupt hotels or other guests.",
            "We may decline or cancel reservations where there is evidence of misuse, and may withdraw access to the service.",
        ],
    },
    Section {
        heading: "6. Listings and accuracy",
        paragraphs: &[
            "Property information and photographs are supplied by hotels and checked during verification. While we take reasonable care, details can change between visits.",
            "If a room materially differs from its listing, tell us. We re-verify the property, and listings that repeatedly mismatch are removed.",
        ],
    },
    Section {
        heading: "7. Liability",
        paragraphs: &[
            "We provide the portal on an 'as is' basis. To the extent permitted by law, we are not liable for the acts or omissions of hotels, for the quality of accommodation, or for loss arising from a hotel's failure to honour a reservation.",
            "Nothing here limits liability for death or personal injury caused by negligence, or for fraud.",
        ],
    },
    Section {
        heading: "8. Changes to these terms",
        paragraphs: &[
            "We may update these terms as the service develops. The version in force is the one published here at the time you make a reservation.",
        ],
    },
];

const PRIVACY: &[Section] = &[
    Section {
        heading: "1. What we collect",
        paragraphs: &[
            "When you reserve a room we collect the lead guest's name, a phone number or email address, your dates, party size and any special requests you add.",
            "We do not collect payment card details. There is nowhere on the portal to enter them.",
            "We keep basic technical information — the pages you visit and general device type — to understand what is used and what is broken.",
        ],
    },
    Section {
        heading: "2. Why we use it",
        paragraphs: &[
            "To pass your reservation to the hotel so it can hold your room and reach you if needed.",
            "To let you retrieve a booking later using your reference and contact details.",
            "To answer support requests, and — only if you opt in — to send occasional deal emails, which you can stop at any time.",
        ],
    },
    Section {
        heading: "3. Who we share it with",
        paragraphs: &[
            "The hotel you reserve with receives your name, contact details, dates, party size and requests. It needs these to honour the reservation.",
            "We do not sell personal data, and we do not share your email address with hotels for marketing purposes.",
            "We may disclose information where required by law or to protect the rights and safety of guests, hotels or our team.",
        ],
    },
    Section {
        heading: "4. Storage on your device",
        paragraphs: &[
            "Your reservations are saved in your browser's local storage so My Reservation works without an account. Clearing your browser data removes them — retrieve them again with your booking reference.",
            "We use only the cookies needed to run the site. There is no advertising or cross-site tracking.",
        ],
    },
    Section {
        heading: "5. How long we keep it",
        paragraphs: &[
            "Reservation records are retained for 24 months after check-out, which covers the period a guest or hotel may need to refer back to a stay.",
            "Support correspondence is kept for 12 months. Marketing subscriptions are kept until you unsubscribe.",
        ],
    },
    Section {
        heading: "6. Your rights",
        paragraphs: &[
            "You can ask for a copy of the personal data we hold about you, ask us to correct it, or ask us to delete it.",
            "Write to privacy@hornofafrica-hotels.com and we will respond within 30 days. Note that a hotel keeps its own record of your stay independently of us.",
        ],
    },
    Section {
        heading: "7. Security",
        paragraphs: &[
            "Data is transmitted over encrypted connections and access is limited to staff who need it to run the service.",
            "Because we hold no payment data, the most sensitive information a breach could expose is a name and a contact number.",
        ],
    },
    Section {
        heading: "8. Contact",
        paragraphs: &[
            "Questions about this policy go to privacy@hornofafrica-hotels.com, or through the contact page.",
        ],
    },
];

#[component]
pub fn TermsPage() -> impl IntoView {
    view! {
        <Title text="Terms & conditions — Horn of Africa Hotel Portal" />
        <LegalDocument
            title="Terms & conditions"
            lede="The agreement between you and the portal when you reserve a room. Plain language, no traps."
            icon="file-text"
            sections=TERMS
            current="Terms & Conditions"
        />
    }
}

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <Title text="Privacy policy — Horn of Africa Hotel Portal" />
        <LegalDocument
            title="Privacy policy"
            lede="What we collect, why, and who sees it. We hold less about you than most booking sites because we never take a card."
            icon="lock"
            sections=PRIVACY
            current="Privacy Policy"
        />
    }
}

#[component]
fn LegalDocument(
    title: &'static str,
    lede: &'static str,
    icon: &'static str,
    sections: &'static [Section],
    current: &'static str,
) -> impl IntoView {
    view! {
        <div class="border-b border-slate-200 bg-gradient-to-br from-slate-50 to-blue-50/50">
            <div class="mx-auto max-w-3xl px-4 py-12">
                <div class="animate-fade-up">
                    <Breadcrumbs trail=vec![
                        ("Home".to_string(), Some("/".to_string())),
                        (current.to_string(), None),
                    ] />
                </div>
                <div class="mt-5 flex animate-fade-up items-start gap-4" style="animation-delay: 70ms">
                    <span class="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl bg-blue-700 text-white shadow-lg shadow-blue-700/25">
                        <Icon name=icon class="h-5 w-5" />
                    </span>
                    <div>
                        <h1 class="text-3xl font-bold tracking-tight text-ink">{title}</h1>
                        <p class="mt-1.5 max-w-xl text-sm leading-relaxed text-slate-600">{lede}</p>
                        <p class="mt-2 flex items-center gap-1.5 text-xs text-slate-400">
                            <Icon name="clock" class="h-3.5 w-3.5" />
                            "Last updated 26 August 2026"
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <div class="mx-auto grid max-w-5xl gap-8 px-4 py-10 lg:grid-cols-[minmax(0,1fr)_15rem]">
            <article class="min-w-0">
                <div class="flex flex-col gap-8">
                    {sections.iter().enumerate().map(|(i, s)| {
                        let anchor = format!("section-{}", i + 1);
                        let delay = format!("animation-delay: {}ms", (i % 4) * 50);
                        view! {
                            <section id=anchor class="reveal scroll-mt-24" style=delay>
                                <h2 class="text-lg font-bold tracking-tight text-ink">{s.heading}</h2>
                                <div class="mt-2.5 flex flex-col gap-3">
                                    {s.paragraphs.iter().map(|p| view! {
                                        <p class="text-sm leading-relaxed text-slate-600">{*p}</p>
                                    }).collect_view()}
                                </div>
                            </section>
                        }
                    }).collect_view()}
                </div>

                <div class="mt-10 flex items-start gap-3 rounded-2xl bg-blue-50 p-5 text-sm text-blue-900 ring-1 ring-blue-100">
                    <Icon name="headset" class="mt-0.5 h-4.5 w-4.5 shrink-0" />
                    <span>
                        <span class="font-bold">"Something unclear? "</span>
                        "Ask us and we will explain it in plain terms — and fix the wording if it needed fixing. "
                        <A href="/contact" attr:class="font-bold underline">"Contact support"</A>
                        "."
                    </span>
                </div>
            </article>

            <nav class="hidden lg:block">
                <div class="sticky top-24 rounded-2xl border border-slate-200 bg-white p-4">
                    <h2 class="mb-3 text-xs font-bold uppercase tracking-wider text-slate-400">"On this page"</h2>
                    <ul class="flex flex-col gap-1">
                        {sections.iter().enumerate().map(|(i, s)| view! {
                            <li>
                                <a
                                    href=format!("#section-{}", i + 1)
                                    class="block rounded-lg px-2.5 py-1.5 text-xs leading-snug text-slate-600 transition-colors hover:bg-blue-50 hover:text-blue-700"
                                >
                                    {s.heading}
                                </a>
                            </li>
                        }).collect_view()}
                    </ul>
                    <div class="mt-3 border-t border-slate-100 pt-3">
                        <A
                            href=if current == "Terms & Conditions" { "/privacy" } else { "/terms" }
                            attr:class="flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-xs font-bold text-blue-700 transition-colors hover:bg-blue-50"
                        >
                            <Icon name="arrow-right" class="h-3 w-3" />
                            {if current == "Terms & Conditions" { "Read the privacy policy" } else { "Read the terms" }}
                        </A>
                    </div>
                </div>
            </nav>
        </div>
    }
}
