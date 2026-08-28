//! Static marketing copy.
//!
//! Everything a hotel publishes — properties, rooms, cities, prices, reviews —
//! comes from the API. What stays here is the portal's own editorial: the
//! testimonials, the FAQ, the how-it-works steps and the headline stats.

#[derive(Debug, Clone, PartialEq)]
pub struct Testimonial {
    pub name: &'static str,
    pub initials: &'static str,
    pub role: &'static str,
    pub quote: &'static str,
    pub rating: u32,
    pub avatar_tint: &'static str,
}

pub const TESTIMONIALS: &[Testimonial] = &[
    Testimonial { name: "Ahmed Hassan", initials: "AH", role: "Business traveller · Addis Ababa", quote: "I booked a room from the taxi on the way in from the airport and had a reference number before I reached the hotel. No card, no account, no fuss.", rating: 5, avatar_tint: "bg-blue-100 text-blue-700" },
    Testimonial { name: "Grace Wanjiru", initials: "GW", role: "Consultant · Nairobi", quote: "I travel across the region constantly and this is the only portal that consistently has accurate phone numbers for the properties. That alone saves me hours.", rating: 5, avatar_tint: "bg-emerald-100 text-emerald-700" },
    Testimonial { name: "Omar Farah", initials: "OF", role: "Visiting family · Hargeisa", quote: "My parents don't use credit cards online. Being able to reserve and pay at the front desk meant I could book for them from London without any trouble.", rating: 5, avatar_tint: "bg-amber-100 text-amber-700" },
    Testimonial { name: "Fatima Ali", initials: "FA", role: "NGO field officer · Mogadishu", quote: "Plans change constantly in my work. Free cancellation on most rooms and a booking reference I can retrieve from any device is exactly what I need.", rating: 4, avatar_tint: "bg-purple-100 text-purple-700" },
    Testimonial { name: "Yusuf Mohamed", initials: "YM", role: "Frequent guest · Toronto", quote: "Prices are the same as calling the hotel directly, which is rare. No inflated commission baked into the rate.", rating: 5, avatar_tint: "bg-rose-100 text-rose-700" },
    Testimonial { name: "Selam Desta", initials: "SD", role: "Event organiser · Dire Dawa", quote: "I reserved eleven rooms for a conference in about ten minutes. The confirmations arrived instantly and every hotel honoured them.", rating: 5, avatar_tint: "bg-cyan-100 text-cyan-700" },
];

#[derive(Debug, Clone, PartialEq)]
pub struct Faq {
    pub question: &'static str,
    pub answer: &'static str,
    pub category: &'static str,
}

pub const FAQS: &[Faq] = &[
    Faq { category: "Booking", question: "Do I need an account to reserve a room?", answer: "No. You reserve with just a name and a phone number or email address. We give you a booking reference immediately, and you can pull the reservation back up on any device using that reference plus the contact details you entered." },
    Faq { category: "Booking", question: "How quickly is my reservation confirmed?", answer: "Instantly. The hotel receives the reservation in their dashboard the moment you submit it, and your booking reference is valid straight away. There is no waiting period and no manual approval step." },
    Faq { category: "Booking", question: "Can I reserve more than one room at a time?", answer: "Yes. Choose the number of rooms in the guests-and-rooms selector before you search, and the reservation form will price the full set together. Every room in the group shares a single booking reference." },
    Faq { category: "Payment", question: "When and how do I pay?", answer: "You pay the hotel directly at check-in, in cash or by card depending on what that property accepts. We never take card details and never charge you online. The price you see is the price the hotel charges." },
    Faq { category: "Payment", question: "Are there any booking fees or commissions?", answer: "None. We do not add a service fee, a booking fee, or a currency conversion margin. Rates shown are the hotel's own published rates for the dates you selected." },
    Faq { category: "Payment", question: "Which currencies are shown?", answer: "Rates are displayed in Ethiopian Birr (ETB) by default. The hotel will charge in its own local currency at the front desk — Kenyan Shillings in Nairobi, Djiboutian Francs in Djibouti City, and so on." },
    Faq { category: "Changes", question: "Can I cancel or change my reservation?", answer: "Most rooms are free to cancel up to 24 hours before check-in — look for the 'Free cancellation' badge on the room. Open your reservation from My Reservation or Retrieve Booking and use Cancel Reservation, or call the hotel directly." },
    Faq { category: "Changes", question: "I lost my booking reference. What now?", answer: "Go to Retrieve Booking and search with the phone number or email you used. If you still cannot find it, contact support with your name and approximate dates and we will trace it with the hotel." },
    Faq { category: "Changes", question: "What if I arrive later than expected?", answer: "Every hotel on the portal has a 24-hour front desk unless its listing says otherwise, so a late arrival is fine. It is still worth calling the hotel — the number is on your confirmation — so they can hold the room." },
    Faq { category: "Hotels", question: "How are hotels verified?", answer: "Every property is visited or video-verified before it goes live, and the contact details are confirmed with the management directly. Listings are re-checked when a hotel updates its rates or photographs." },
    Faq { category: "Hotels", question: "Are the photographs accurate?", answer: "Photographs are supplied by the hotel and checked against the verification visit. If a room does not match its listing, tell us and we will re-verify the property — persistent mismatches get a listing removed." },
    Faq { category: "Hotels", question: "How do I list my hotel on the portal?", answer: "Hotels manage their own rooms, rates and reservations through the Tourista admin dashboard. Get in touch through the contact page and we will walk your team through onboarding, usually within a couple of days." },
];

#[derive(Debug, Clone, PartialEq)]
pub struct HowItWorksStep {
    pub number: &'static str,
    pub icon: &'static str,
    pub title: &'static str,
    pub body: &'static str,
}

pub const HOW_IT_WORKS: &[HowItWorksStep] = &[
    HowItWorksStep { number: "01", icon: "search", title: "Search", body: "Pick your city and dates. We show every verified hotel with live availability — no sponsored placements pushed to the top." },
    HowItWorksStep { number: "02", icon: "bed", title: "Choose your room", body: "Compare room types side by side with real photographs, exact sizes, bed configurations and what each rate includes." },
    HowItWorksStep { number: "03", icon: "user-check", title: "Reserve in seconds", body: "Just a name and a contact number. No account to create, no card details, no verification email to chase." },
    HowItWorksStep { number: "04", icon: "key", title: "Pay at the hotel", body: "Show your booking reference at the front desk and settle the bill there. Exactly the rate you were quoted." },
];

#[derive(Debug, Clone, PartialEq)]
pub struct Stat {
    pub value: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
}

/// The four figures the home and about pages show. Values are filled in from
/// [`crate::api::PortalStats`] at render time — see [`Stat::value_from`] — so the
/// site never advertises inventory it does not have.
pub const STATS: &[Stat] = &[
    Stat { value: "hotels", label: "Verified hotels", icon: "building" },
    Stat { value: "countries", label: "Countries covered", icon: "globe" },
    Stat { value: "rooms", label: "Rooms listed", icon: "bed" },
    Stat { value: "rating", label: "Average guest rating", icon: "star" },
];

impl Stat {
    /// Resolves this tile against the live portal figures.
    pub fn value_from(&self, stats: &crate::api::PortalStats) -> String {
        match self.value {
            "hotels" => stats.hotels_display(),
            "countries" => stats.countries.to_string(),
            "rooms" => stats.rooms_display(),
            "rating" if stats.average_rating > 0.0 => format!("{:.1}", stats.average_rating),
            "rating" => "—".to_string(),
            other => other.to_string(),
        }
    }
}
