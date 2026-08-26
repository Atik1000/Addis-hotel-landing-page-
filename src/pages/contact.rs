use crate::components::{use_toast, Icon};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const TOPICS: [&str; 5] = [
    "A question about my reservation",
    "I can't find my booking",
    "I want to list my hotel",
    "Report a problem with a listing",
    "Something else",
];

#[component]
pub fn ContactPage() -> impl IntoView {
    let toast = use_toast();
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let topic = RwSignal::new(TOPICS[0].to_string());
    let reference = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());
    let sent = RwSignal::new(false);
    let errors = RwSignal::new(Vec::<String>::new());

    let submit = move |_| {
        let mut found = Vec::new();
        if name.get().trim().len() < 2 {
            found.push("Tell us your name.".to_string());
        }
        let e = email.get();
        if !e.contains('@') || !e.contains('.') {
            found.push("Enter an email address we can reply to.".to_string());
        }
        if message.get().trim().len() < 15 {
            found.push("Add a little more detail so we can help properly.".to_string());
        }
        errors.set(found.clone());

        if found.is_empty() {
            sent.set(true);
            toast.success("Message sent", "We usually reply within a few hours during business hours.");
        } else {
            toast.error("Check the form", found[0].clone());
        }
    };

    let field = "w-full rounded-xl border border-slate-300 px-3.5 py-3 text-sm text-slate-800 transition-all duration-200 focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100";

    view! {
        <Title text="Contact us — Horn of Africa Hotel Portal" />

        <section class="border-b border-slate-200 bg-gradient-to-br from-blue-50 via-white to-indigo-50">
            <div class="mx-auto max-w-3xl px-4 py-14 text-center">
                <span class="mx-auto flex h-14 w-14 animate-scale-in items-center justify-center rounded-2xl bg-blue-700 text-white shadow-xl shadow-blue-700/25">
                    <Icon name="headset" class="h-6 w-6" />
                </span>
                <h1 class="mt-5 animate-fade-up text-3xl font-extrabold tracking-tight text-slate-900 sm:text-4xl" style="animation-delay: 70ms">
                    "Talk to us"
                </h1>
                <p class="mx-auto mt-3 max-w-lg animate-fade-up text-sm leading-relaxed text-slate-600" style="animation-delay: 120ms">
                    "Guests, hoteliers and anyone who has spotted something wrong with a listing — this reaches the same small team."
                </p>
            </div>
        </section>

        <div class="mx-auto grid max-w-5xl gap-8 px-4 py-12 lg:grid-cols-[minmax(0,1fr)_20rem]">
            // ---- Form ---------------------------------------------------
            <div class="min-w-0">
                <Show
                    when=move || sent.get()
                    fallback=move || view! {
                        <div class="animate-fade-up rounded-2xl border border-slate-200 bg-white p-6 shadow-sm">
                            <h2 class="text-lg font-bold text-slate-900">"Send us a message"</h2>
                            <p class="mt-1 text-sm text-slate-500">"Fields marked with * are required."</p>

                            <Show when=move || !errors.get().is_empty()>
                                <div class="mt-4 rounded-xl border border-red-200 bg-red-50 p-4">
                                    <p class="flex items-center gap-2 text-sm font-bold text-red-800">
                                        <Icon name="alert" class="h-4 w-4" />
                                        "Please fix the following"
                                    </p>
                                    <ul class="mt-2 flex flex-col gap-1 pl-6 text-sm text-red-700">
                                        {move || errors.get().into_iter().map(|e| view! {
                                            <li class="list-disc">{e}</li>
                                        }).collect_view()}
                                    </ul>
                                </div>
                            </Show>

                            <div class="mt-5 flex flex-col gap-4">
                                <div class="grid gap-4 sm:grid-cols-2">
                                    <div>
                                        <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                                            "Your name" <span class="text-red-500">"*"</span>
                                        </label>
                                        <input
                                            type="text"
                                            placeholder="Ahmed Hassan"
                                            class=field
                                            prop:value=name
                                            on:input:target=move |ev| name.set(ev.target().value())
                                        />
                                    </div>
                                    <div>
                                        <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                                            "Email" <span class="text-red-500">"*"</span>
                                        </label>
                                        <input
                                            type="email"
                                            placeholder="you@example.com"
                                            class=field
                                            prop:value=email
                                            on:input:target=move |ev| email.set(ev.target().value())
                                        />
                                    </div>
                                </div>

                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">"What is this about?"</label>
                                    <select class=field on:change:target=move |ev| topic.set(ev.target().value())>
                                        {TOPICS.into_iter().map(|t| view! {
                                            <option selected=move || (topic.get() == t)>{t}</option>
                                        }).collect_view()}
                                    </select>
                                </div>

                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                                        "Booking reference " <span class="font-medium normal-case text-slate-400">"(if you have one)"</span>
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="HA-250143"
                                        class=format!("{field} font-mono uppercase")
                                        prop:value=reference
                                        on:input:target=move |ev| reference.set(ev.target().value())
                                    />
                                </div>

                                <div>
                                    <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                                        "Message" <span class="text-red-500">"*"</span>
                                    </label>
                                    <textarea
                                        rows="5"
                                        placeholder="Tell us what happened, including dates and the hotel name if it's about a stay."
                                        class=format!("{field} resize-none")
                                        prop:value=message
                                        on:input:target=move |ev| message.set(ev.target().value())
                                    ></textarea>
                                    <p class="mt-1.5 text-xs text-slate-400">
                                        {move || format!("{} characters", message.get().chars().count())}
                                    </p>
                                </div>

                                <button
                                    on:click=submit
                                    class="sheen flex w-full items-center justify-center gap-2 rounded-xl bg-blue-700 py-3.5 text-sm font-bold text-white shadow-lg shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-xl active:scale-[0.98]"
                                >
                                    <Icon name="send" class="h-4 w-4" />
                                    "Send message"
                                </button>

                                <p class="flex items-center justify-center gap-1.5 text-xs text-slate-400">
                                    <Icon name="lock" class="h-3.5 w-3.5" />
                                    "We only use your details to answer this message."
                                </p>
                            </div>
                        </div>
                    }
                >
                    <div class="flex animate-scale-in flex-col items-center gap-3 rounded-2xl border border-emerald-200 bg-emerald-50 px-6 py-14 text-center">
                        <span class="relative flex h-16 w-16 items-center justify-center">
                            <span class="absolute inset-0 animate-pulse-ring rounded-full bg-emerald-400"></span>
                            <span class="relative flex h-16 w-16 items-center justify-center rounded-full bg-emerald-500 text-white shadow-lg">
                                <Icon name="check" class="h-7 w-7" />
                            </span>
                        </span>
                        <h2 class="mt-2 text-xl font-extrabold text-slate-900">"Message sent"</h2>
                        <p class="max-w-sm text-sm leading-relaxed text-slate-600">
                            {move || format!(
                                "Thanks {}. We've got your message about \"{}\" and will reply to {} shortly.",
                                name.get(), topic.get(), email.get()
                            )}
                        </p>
                        <div class="mt-3 flex flex-wrap justify-center gap-2.5">
                            <button
                                on:click=move |_| {
                                    sent.set(false);
                                    message.set(String::new());
                                    reference.set(String::new());
                                }
                                class="rounded-xl border border-slate-300 bg-white px-5 py-2.5 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50"
                            >
                                "Send another"
                            </button>
                            <A
                                href="/"
                                attr:class="rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800"
                            >
                                "Back to home"
                            </A>
                        </div>
                    </div>
                </Show>
            </div>

            // ---- Sidebar ------------------------------------------------
            <aside class="flex flex-col gap-4">
                <div class="animate-slide-in-right rounded-2xl border border-slate-200 bg-white p-5">
                    <h2 class="text-sm font-bold text-slate-900">"Other ways to reach us"</h2>
                    <div class="mt-3.5 flex flex-col gap-2.5">
                        <a
                            href="mailto:support@hornofafrica-hotels.com"
                            class="group flex items-center gap-3 rounded-xl border border-slate-200 p-3 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:shadow-sm"
                        >
                            <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-700">
                                <Icon name="mail" class="h-4 w-4" />
                            </span>
                            <span class="min-w-0">
                                <span class="block text-xs text-slate-400">"Email"</span>
                                <span class="block truncate text-sm font-semibold text-slate-800">"support@hornofafrica-hotels.com"</span>
                            </span>
                        </a>
                        <a
                            href="tel:+251111234567"
                            class="group flex items-center gap-3 rounded-xl border border-slate-200 p-3 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:shadow-sm"
                        >
                            <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-700">
                                <Icon name="phone" class="h-4 w-4" />
                            </span>
                            <span class="min-w-0">
                                <span class="block text-xs text-slate-400">"Phone"</span>
                                <span class="block text-sm font-semibold text-slate-800">"+251 11 123 4567"</span>
                            </span>
                        </a>
                        <a
                            href="https://wa.me/251912345678"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="group flex items-center gap-3 rounded-xl border border-slate-200 p-3 transition-all duration-200 hover:-translate-y-0.5 hover:border-emerald-300 hover:shadow-sm"
                        >
                            <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-emerald-50 text-emerald-600">
                                <Icon name="message" class="h-4 w-4" />
                            </span>
                            <span class="min-w-0">
                                <span class="block text-xs text-slate-400">"WhatsApp"</span>
                                <span class="block text-sm font-semibold text-slate-800">"+251 91 234 5678"</span>
                            </span>
                        </a>
                    </div>
                </div>

                <div class="animate-slide-in-right rounded-2xl border border-slate-200 bg-white p-5" style="animation-delay: 80ms">
                    <h2 class="flex items-center gap-2 text-sm font-bold text-slate-900">
                        <Icon name="clock" class="h-4 w-4 text-blue-700" />
                        "Support hours"
                    </h2>
                    <dl class="mt-3 flex flex-col gap-2 text-sm">
                        {[
                            ("Monday – Friday", "8:00 – 20:00"),
                            ("Saturday", "9:00 – 17:00"),
                            ("Sunday", "10:00 – 16:00"),
                        ].into_iter().map(|(day, hours)| view! {
                            <div class="flex justify-between gap-3">
                                <dt class="text-slate-500">{day}</dt>
                                <dd class="font-semibold text-slate-800">{hours}</dd>
                            </div>
                        }).collect_view()}
                    </dl>
                    <p class="mt-3 border-t border-slate-100 pt-3 text-xs leading-relaxed text-slate-400">
                        "East Africa Time (EAT). Messages outside these hours are answered the next working morning."
                    </p>
                </div>

                <div class="animate-slide-in-right rounded-2xl bg-blue-50 p-5 ring-1 ring-blue-100" style="animation-delay: 140ms">
                    <h2 class="flex items-center gap-2 text-sm font-bold text-blue-900">
                        <Icon name="sparkles" class="h-4 w-4" />
                        "Faster than emailing"
                    </h2>
                    <p class="mt-2 text-xs leading-relaxed text-blue-800">
                        "For anything about an existing stay — a late arrival, an extra bed, a room preference — calling the hotel directly is usually quickest. The number is on your confirmation."
                    </p>
                    <A
                        href="/my-reservations"
                        attr:class="mt-3 flex items-center justify-center gap-1.5 rounded-xl bg-white py-2.5 text-sm font-bold text-blue-700 shadow-sm transition-transform hover:-translate-y-0.5"
                    >
                        <Icon name="ticket" class="h-3.5 w-3.5" />
                        "Open my reservations"
                    </A>
                </div>
            </aside>
        </div>
    }
}
