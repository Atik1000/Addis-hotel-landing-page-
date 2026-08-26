//! Static demo content for the portal.
//!
//! Everything here is `const` so pages can hold `&'static` references without
//! cloning — there is no backend behind this build yet.

#[derive(Debug, Clone, PartialEq)]
pub struct RoomType {
    pub id: &'static str,
    pub name: &'static str,
    pub beds: &'static str,
    pub guests: u32,
    pub size_sqm: u32,
    pub price_per_night: u32,
    pub image: &'static str,
    pub perks: &'static [&'static str],
    /// Rooms left at this rate — drives the "only N left" urgency chip.
    pub rooms_left: u32,
    pub refundable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Review {
    pub author: &'static str,
    pub initials: &'static str,
    pub country: &'static str,
    pub rating: u32,
    pub date: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub stayed_in: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Landmark {
    pub name: &'static str,
    pub distance: &'static str,
    pub icon: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hotel {
    pub id: &'static str,
    pub name: &'static str,
    pub area: &'static str,
    pub city: &'static str,
    pub country: &'static str,
    pub rating: f32,
    pub review_count: u32,
    pub price_from: u32,
    /// Struck-through "was" price, when the hotel is running a promotion.
    pub price_was: Option<u32>,
    pub star_class: u32,
    pub image: &'static str,
    pub tags: &'static [&'static str],
    pub amenities: &'static [&'static str],
    pub description: &'static str,
    pub phone: &'static str,
    pub whatsapp: &'static str,
    pub photos: &'static [&'static str],
    pub rooms: &'static [RoomType],
    pub reviews: &'static [Review],
    pub landmarks: &'static [Landmark],
    pub check_in_time: &'static str,
    pub check_out_time: &'static str,
    pub featured: bool,
    /// Short marketing line shown on featured cards.
    pub highlight: &'static str,
}

impl Hotel {
    /// Star distribution for the ratings breakdown bar chart, newest-first.
    pub fn rating_breakdown(&self) -> [u32; 5] {
        let mut counts = [0u32; 5];
        for r in self.reviews {
            let idx = (r.rating.clamp(1, 5) - 1) as usize;
            counts[idx] += 1;
        }
        // Scale the sample up to the advertised review count so the bars look
        // proportional rather than reflecting only the handful shown.
        let sample: u32 = counts.iter().sum();
        if sample == 0 || self.review_count == 0 {
            return counts;
        }
        let mut scaled = [0u32; 5];
        for i in 0..5 {
            scaled[i] = counts[i] * self.review_count / sample;
        }
        scaled
    }

    pub fn location_line(&self) -> String {
        format!("{}, {}, {}", self.area, self.city, self.country)
    }

    pub fn cheapest_room(&self) -> Option<&'static RoomType> {
        self.rooms.iter().min_by_key(|r| r.price_per_night)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct City {
    pub name: &'static str,
    pub country: &'static str,
    pub image: &'static str,
    pub hotel_count: u32,
}

pub const CITIES: &[City] = &[
    City { name: "Addis Ababa", country: "Ethiopia", image: "https://images.unsplash.com/photo-1553913861-c0fddf2619ee?q=80&w=600", hotel_count: 214 },
    City { name: "Mogadishu", country: "Somalia", image: "https://images.unsplash.com/photo-1477959858617-67f85cf4f1df?q=80&w=600", hotel_count: 68 },
    City { name: "Hargeisa", country: "Somaliland", image: "https://images.unsplash.com/photo-1500534623283-312aade485b7?q=80&w=600", hotel_count: 41 },
    City { name: "Nairobi", country: "Kenya", image: "https://images.unsplash.com/photo-1611348586804-61bf6c080437?q=80&w=600", hotel_count: 302 },
    City { name: "Dire Dawa", country: "Ethiopia", image: "https://images.unsplash.com/photo-1470770903676-69b98201ea1c?q=80&w=600", hotel_count: 37 },
    City { name: "Djibouti City", country: "Djibouti", image: "https://images.unsplash.com/photo-1526772662000-3f88f10405ff?q=80&w=600", hotel_count: 29 },
];

// ---------------------------------------------------------------------------
// Room sets
// ---------------------------------------------------------------------------

const ROOMS_UPSCALE: &[RoomType] = &[
    RoomType { id: "standard", name: "Standard Room", beds: "1 Queen Bed", guests: 2, size_sqm: 20, price_per_night: 2800, image: "https://images.unsplash.com/photo-1611892440504-42a792e24d32?q=80&w=800", perks: &["Free Wi-Fi", "Breakfast included", "City view"], rooms_left: 6, refundable: true },
    RoomType { id: "deluxe", name: "Deluxe Room", beds: "1 King Bed", guests: 2, size_sqm: 28, price_per_night: 3600, image: "https://images.unsplash.com/photo-1590490360182-c33d57733427?q=80&w=800", perks: &["Free Wi-Fi", "Breakfast included", "Balcony", "Minibar"], rooms_left: 3, refundable: true },
    RoomType { id: "executive", name: "Executive Suite", beds: "1 King Bed + 1 Sofa Bed", guests: 4, size_sqm: 40, price_per_night: 4800, image: "https://images.unsplash.com/photo-1582719478250-c89cae4dc85b?q=80&w=800", perks: &["Free Wi-Fi", "Breakfast included", "Lounge access", "Late check-out"], rooms_left: 2, refundable: false },
];

const ROOMS_MID: &[RoomType] = &[
    RoomType { id: "single", name: "Single Room", beds: "1 Single Bed", guests: 1, size_sqm: 16, price_per_night: 1500, image: "https://images.unsplash.com/photo-1631049307264-da0ec9d70304?q=80&w=800", perks: &["Free Wi-Fi", "Work desk"], rooms_left: 9, refundable: true },
    RoomType { id: "twin", name: "Twin Room", beds: "2 Single Beds", guests: 2, size_sqm: 22, price_per_night: 2100, image: "https://images.unsplash.com/photo-1566665797739-1674de7a421a?q=80&w=800", perks: &["Free Wi-Fi", "Breakfast included"], rooms_left: 5, refundable: true },
    RoomType { id: "family", name: "Family Room", beds: "1 King + 2 Singles", guests: 4, size_sqm: 34, price_per_night: 3200, image: "https://images.unsplash.com/photo-1618773928121-c32242e63f39?q=80&w=800", perks: &["Free Wi-Fi", "Breakfast included", "Extra bed available"], rooms_left: 4, refundable: true },
];

const ROOMS_LUXURY: &[RoomType] = &[
    RoomType { id: "deluxe", name: "Deluxe King", beds: "1 King Bed", guests: 2, size_sqm: 32, price_per_night: 5200, image: "https://images.unsplash.com/photo-1595576508898-0ad5c879a061?q=80&w=800", perks: &["Free Wi-Fi", "Breakfast included", "Pool access", "Nespresso"], rooms_left: 4, refundable: true },
    RoomType { id: "junior-suite", name: "Junior Suite", beds: "1 King Bed", guests: 3, size_sqm: 45, price_per_night: 7400, image: "https://images.unsplash.com/photo-1631049035182-249067d7618e?q=80&w=800", perks: &["Free Wi-Fi", "Lounge access", "Airport transfer", "Butler service"], rooms_left: 2, refundable: true },
    RoomType { id: "presidential", name: "Presidential Suite", beds: "1 King Bed + Living Room", guests: 4, size_sqm: 92, price_per_night: 14800, image: "https://images.unsplash.com/photo-1591088398332-8a7791972843?q=80&w=800", perks: &["Private terrace", "Dedicated butler", "Airport transfer", "Spa credit"], rooms_left: 1, refundable: false },
];

const ROOMS_BUDGET: &[RoomType] = &[
    RoomType { id: "economy", name: "Economy Room", beds: "1 Double Bed", guests: 2, size_sqm: 14, price_per_night: 900, image: "https://images.unsplash.com/photo-1522708323590-d24dbb6b0267?q=80&w=800", perks: &["Free Wi-Fi"], rooms_left: 12, refundable: true },
    RoomType { id: "standard", name: "Standard Room", beds: "1 Double Bed", guests: 2, size_sqm: 18, price_per_night: 1300, image: "https://images.unsplash.com/photo-1560448204-e02f11c3d0e2?q=80&w=800", perks: &["Free Wi-Fi", "Breakfast included"], rooms_left: 7, refundable: true },
];

// ---------------------------------------------------------------------------
// Reviews
// ---------------------------------------------------------------------------

const REVIEWS_GOLDEN_TULIP: &[Review] = &[
    Review { author: "Ahmed Hassan", initials: "AH", country: "Ethiopia", rating: 5, date: "12 August 2026", title: "Faultless from check-in to check-out", body: "The staff remembered my name from a previous stay, the room was immaculate and the airport shuttle was waiting exactly when promised. This is now my default hotel in Addis.", stayed_in: "Deluxe Room · 2 nights" },
    Review { author: "Grace Wanjiru", initials: "GW", country: "Kenya", rating: 5, date: "3 August 2026", title: "Great value for a business trip", body: "Fast Wi-Fi that actually held a video call, a proper work desk and breakfast from 6am. Everything a working traveller needs.", stayed_in: "Executive Suite · 3 nights" },
    Review { author: "Omar Farah", initials: "OF", country: "United Kingdom", rating: 4, date: "28 July 2026", title: "Lovely rooms, breakfast could be wider", body: "Rooms are quiet and modern and the pool is a real bonus. The breakfast buffet is good but repeats itself if you stay more than two nights.", stayed_in: "Standard Room · 4 nights" },
    Review { author: "Selam Desta", initials: "SD", country: "Ethiopia", rating: 5, date: "19 July 2026", title: "The service makes it", body: "Booked at short notice for family visiting from abroad. Reception arranged a connecting room without being asked. Genuinely warm hospitality.", stayed_in: "Deluxe Room · 1 night" },
    Review { author: "Mohamed Nur", initials: "MN", country: "United States", rating: 4, date: "8 July 2026", title: "Excellent location", body: "Ten minutes from Bole and walking distance to plenty of restaurants. Traffic noise is noticeable in the lower floors — ask for something high up.", stayed_in: "Standard Room · 2 nights" },
];

const REVIEWS_GENERIC: &[Review] = &[
    Review { author: "Fatima Ali", initials: "FA", country: "Ethiopia", rating: 5, date: "10 August 2026", title: "Clean, calm and well run", body: "Checked in late at night and the front desk had everything ready. Room was spotless and the bed was excellent.", stayed_in: "Twin Room · 2 nights" },
    Review { author: "Yusuf Mohamed", initials: "YM", country: "Canada", rating: 4, date: "1 August 2026", title: "Solid choice, fair price", body: "Nothing flashy but everything worked, and the price is hard to argue with for the area. Would book again.", stayed_in: "Standard Room · 3 nights" },
    Review { author: "Hawa Abdi", initials: "HA", country: "Somalia", rating: 5, date: "22 July 2026", title: "Very helpful staff", body: "They arranged a taxi at 4am for my flight and packed a breakfast box without me even asking. Small things that matter.", stayed_in: "Family Room · 2 nights" },
    Review { author: "Bereket Tesfaye", initials: "BT", country: "Ethiopia", rating: 3, date: "14 July 2026", title: "Good, with a couple of niggles", body: "Room and service were fine. Hot water took a while in the mornings and the lift was slow at peak times.", stayed_in: "Single Room · 1 night" },
];

// ---------------------------------------------------------------------------
// Landmarks
// ---------------------------------------------------------------------------

const LANDMARKS_ADDIS: &[Landmark] = &[
    Landmark { name: "Bole International Airport", distance: "4.2 km", icon: "plane" },
    Landmark { name: "Edna Mall", distance: "1.1 km", icon: "tag" },
    Landmark { name: "Friendship City Center", distance: "2.4 km", icon: "building" },
    Landmark { name: "Unity Park", distance: "6.8 km", icon: "map-pin" },
    Landmark { name: "National Museum", distance: "7.5 km", icon: "award" },
];

const LANDMARKS_GENERIC: &[Landmark] = &[
    Landmark { name: "City Centre", distance: "1.8 km", icon: "building" },
    Landmark { name: "International Airport", distance: "9.0 km", icon: "plane" },
    Landmark { name: "Central Market", distance: "2.2 km", icon: "tag" },
    Landmark { name: "Main Bus Terminal", distance: "3.4 km", icon: "shuttle" },
];

// ---------------------------------------------------------------------------
// Hotels
// ---------------------------------------------------------------------------

pub const HOTELS: &[Hotel] = &[
    Hotel {
        id: "golden-tulip-addis-ababa",
        name: "Golden Tulip Addis Ababa",
        area: "Bole", city: "Addis Ababa", country: "Ethiopia",
        rating: 4.4, review_count: 128, price_from: 2800, price_was: Some(3400), star_class: 4,
        image: "https://images.unsplash.com/photo-1566073771259-6a8506099945?q=80&w=1200",
        tags: &["Wi-Fi", "Breakfast", "Airport Shuttle", "Pool"],
        amenities: &["Free Wi-Fi", "Airport Pickup", "Restaurant", "Parking", "24/7 Front Desk", "Swimming Pool", "Gym", "Bar", "Room Service", "Laundry", "Family Rooms", "Wheelchair Access", "Air Conditioning", "Business Centre"],
        description: "Golden Tulip Addis Ababa pairs contemporary rooms with genuinely warm Ethiopian hospitality, minutes from Bole International Airport. The hotel is built around a landscaped courtyard pool, with an all-day restaurant serving both Ethiopian and international menus, a fully equipped gym and meeting rooms for up to 120 guests. Rooms are soundproofed, air conditioned and come with fast Wi-Fi that holds up to video calls — the reason it is a standing favourite with business travellers passing through the city.",
        phone: "+251 11 123 4567", whatsapp: "+251 91 234 5678",
        photos: &[
            "https://images.unsplash.com/photo-1566073771259-6a8506099945?q=80&w=1200",
            "https://images.unsplash.com/photo-1611892440504-42a792e24d32?q=80&w=1200",
            "https://images.unsplash.com/photo-1590490360182-c33d57733427?q=80&w=1200",
            "https://images.unsplash.com/photo-1582719478250-c89cae4dc85b?q=80&w=1200",
            "https://images.unsplash.com/photo-1571003123894-1f0594d2b5d9?q=80&w=1200",
            "https://images.unsplash.com/photo-1540541338287-41700207dee6?q=80&w=1200",
            "https://images.unsplash.com/photo-1584132967334-10e028bd69f7?q=80&w=1200",
        ],
        rooms: ROOMS_UPSCALE, reviews: REVIEWS_GOLDEN_TULIP, landmarks: LANDMARKS_ADDIS,
        check_in_time: "2:00 PM", check_out_time: "12:00 PM", featured: true,
        highlight: "Courtyard pool · 4 km from Bole Airport",
    },
    Hotel {
        id: "haile-grand-addis-hotel",
        name: "Haile Grand Addis Hotel",
        area: "Kazanchis", city: "Addis Ababa", country: "Ethiopia",
        rating: 4.3, review_count: 96, price_from: 2100, price_was: None, star_class: 4,
        image: "https://images.unsplash.com/photo-1551882547-ff40c63fe5fa?q=80&w=1200",
        tags: &["Wi-Fi", "Breakfast", "Parking", "Gym"],
        amenities: &["Free Wi-Fi", "Parking", "Restaurant", "24/7 Front Desk", "Gym", "Bar", "Room Service", "Laundry", "Air Conditioning"],
        description: "A landmark property in Kazanchis with generously sized rooms and panoramic views over the city. The rooftop restaurant is a destination in its own right, and the location puts the UN compound and the main conference centre within a short walk.",
        phone: "+251 11 222 3344", whatsapp: "+251 91 222 3344",
        photos: &[
            "https://images.unsplash.com/photo-1551882547-ff40c63fe5fa?q=80&w=1200",
            "https://images.unsplash.com/photo-1618773928121-c32242e63f39?q=80&w=1200",
            "https://images.unsplash.com/photo-1566665797739-1674de7a421a?q=80&w=1200",
            "https://images.unsplash.com/photo-1631049307264-da0ec9d70304?q=80&w=1200",
        ],
        rooms: ROOMS_MID, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_ADDIS,
        check_in_time: "2:00 PM", check_out_time: "12:00 PM", featured: true,
        highlight: "Rooftop restaurant with city views",
    },
    Hotel {
        id: "ellilly-international-hotel",
        name: "Ellilly International Hotel",
        area: "Kirkos", city: "Addis Ababa", country: "Ethiopia",
        rating: 4.2, review_count: 80, price_from: 1800, price_was: Some(2200), star_class: 3,
        image: "https://images.unsplash.com/photo-1520250497591-112f2f40a3f4?q=80&w=1200",
        tags: &["Wi-Fi", "Breakfast", "Airport Shuttle"],
        amenities: &["Free Wi-Fi", "Airport Pickup", "Restaurant", "24/7 Front Desk", "Laundry", "Air Conditioning"],
        description: "Comfortable, straightforward rooms a short drive from the city centre, popular with guests who want a reliable base without a luxury price tag. Breakfast is included and the airport shuttle runs on request.",
        phone: "+251 11 333 4455", whatsapp: "+251 91 333 4455",
        photos: &[
            "https://images.unsplash.com/photo-1520250497591-112f2f40a3f4?q=80&w=1200",
            "https://images.unsplash.com/photo-1560448204-e02f11c3d0e2?q=80&w=1200",
            "https://images.unsplash.com/photo-1522708323590-d24dbb6b0267?q=80&w=1200",
        ],
        rooms: ROOMS_MID, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_ADDIS,
        check_in_time: "1:00 PM", check_out_time: "11:00 AM", featured: false,
        highlight: "Best value in Kirkos",
    },
    Hotel {
        id: "harmony-hotel",
        name: "Harmony Hotel",
        area: "Bole", city: "Addis Ababa", country: "Ethiopia",
        rating: 4.2, review_count: 74, price_from: 2400, price_was: None, star_class: 4,
        image: "https://images.unsplash.com/photo-1445019980597-93fa8acb246c?q=80&w=1200",
        tags: &["Wi-Fi", "Breakfast", "Parking", "Bar"],
        amenities: &["Free Wi-Fi", "Parking", "Restaurant", "Bar", "Gym", "Room Service", "Air Conditioning"],
        description: "Harmony Hotel blends contemporary design with attentive service, minutes from Bole's restaurants and nightlife. Rooms are bright and quiet, with blackout curtains and a proper desk in every category.",
        phone: "+251 11 444 5566", whatsapp: "+251 91 444 5566",
        photos: &[
            "https://images.unsplash.com/photo-1445019980597-93fa8acb246c?q=80&w=1200",
            "https://images.unsplash.com/photo-1590490360182-c33d57733427?q=80&w=1200",
            "https://images.unsplash.com/photo-1595576508898-0ad5c879a061?q=80&w=1200",
        ],
        rooms: ROOMS_UPSCALE, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_ADDIS,
        check_in_time: "2:00 PM", check_out_time: "12:00 PM", featured: false,
        highlight: "Walking distance to Bole nightlife",
    },
    Hotel {
        id: "skylight-hotel-addis",
        name: "Skylight Hotel Addis",
        area: "Airport Road", city: "Addis Ababa", country: "Ethiopia",
        rating: 4.6, review_count: 241, price_from: 5200, price_was: Some(6100), star_class: 5,
        image: "https://images.unsplash.com/photo-1542314831-068cd1dbfeeb?q=80&w=1200",
        tags: &["Wi-Fi", "Pool", "Spa", "Airport Shuttle"],
        amenities: &["Free Wi-Fi", "Airport Pickup", "Restaurant", "Parking", "24/7 Front Desk", "Swimming Pool", "Gym", "Spa", "Bar", "Room Service", "Laundry", "Business Centre", "Air Conditioning", "Wheelchair Access", "Family Rooms"],
        description: "The city's largest five-star property, sitting directly opposite the airport terminal. Six restaurants, a full spa, a 25-metre pool and conference space for well over a thousand delegates. Rooms are large by any standard and the soundproofing is exceptional given the location.",
        phone: "+251 11 555 6677", whatsapp: "+251 91 555 6677",
        photos: &[
            "https://images.unsplash.com/photo-1542314831-068cd1dbfeeb?q=80&w=1200",
            "https://images.unsplash.com/photo-1591088398332-8a7791972843?q=80&w=1200",
            "https://images.unsplash.com/photo-1631049035182-249067d7618e?q=80&w=1200",
            "https://images.unsplash.com/photo-1571896349842-33c89424de2d?q=80&w=1200",
            "https://images.unsplash.com/photo-1584132915807-fd1f5fbc078f?q=80&w=1200",
        ],
        rooms: ROOMS_LUXURY, reviews: REVIEWS_GOLDEN_TULIP, landmarks: LANDMARKS_ADDIS,
        check_in_time: "3:00 PM", check_out_time: "12:00 PM", featured: true,
        highlight: "Five-star spa · Opposite the airport terminal",
    },
    Hotel {
        id: "cactus-hotel-addis",
        name: "Cactus Hotel & Suites",
        area: "Sarbet", city: "Addis Ababa", country: "Ethiopia",
        rating: 3.9, review_count: 52, price_from: 1300, price_was: None, star_class: 3,
        image: "https://images.unsplash.com/photo-1512918728675-ed5a9ecdebfd?q=80&w=1200",
        tags: &["Wi-Fi", "Parking", "Breakfast"],
        amenities: &["Free Wi-Fi", "Parking", "Restaurant", "24/7 Front Desk", "Laundry"],
        description: "A friendly, no-frills hotel in Sarbet with clean rooms and a small courtyard café. Good for longer stays — weekly rates are available on request at the front desk.",
        phone: "+251 11 666 7788", whatsapp: "+251 91 666 7788",
        photos: &[
            "https://images.unsplash.com/photo-1512918728675-ed5a9ecdebfd?q=80&w=1200",
            "https://images.unsplash.com/photo-1522708323590-d24dbb6b0267?q=80&w=1200",
        ],
        rooms: ROOMS_BUDGET, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_ADDIS,
        check_in_time: "12:00 PM", check_out_time: "11:00 AM", featured: false,
        highlight: "Weekly rates for long stays",
    },
    Hotel {
        id: "jazeera-palace-mogadishu",
        name: "Jazeera Palace Hotel",
        area: "Wadajir", city: "Mogadishu", country: "Somalia",
        rating: 4.5, review_count: 113, price_from: 4200, price_was: None, star_class: 4,
        image: "https://images.unsplash.com/photo-1564501049412-61c2a3083791?q=80&w=1200",
        tags: &["Wi-Fi", "Breakfast", "Airport Shuttle", "Security"],
        amenities: &["Free Wi-Fi", "Airport Pickup", "Restaurant", "Parking", "24/7 Front Desk", "Gym", "Room Service", "Laundry", "Air Conditioning", "Business Centre"],
        description: "Mogadishu's best-known international-standard hotel, close to the airport with comprehensive security arrangements. Rooms are spacious and well maintained, and the restaurant serves a strong Somali and Middle Eastern menu.",
        phone: "+252 61 123 4567", whatsapp: "+252 61 123 4567",
        photos: &[
            "https://images.unsplash.com/photo-1564501049412-61c2a3083791?q=80&w=1200",
            "https://images.unsplash.com/photo-1631049307264-da0ec9d70304?q=80&w=1200",
            "https://images.unsplash.com/photo-1595576508898-0ad5c879a061?q=80&w=1200",
        ],
        rooms: ROOMS_UPSCALE, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_GENERIC,
        check_in_time: "2:00 PM", check_out_time: "12:00 PM", featured: true,
        highlight: "Airport-adjacent with full security",
    },
    Hotel {
        id: "peace-hotel-mogadishu",
        name: "Peace Hotel Mogadishu",
        area: "Hodan", city: "Mogadishu", country: "Somalia",
        rating: 4.0, review_count: 47, price_from: 2600, price_was: Some(3100), star_class: 3,
        image: "https://images.unsplash.com/photo-1618773928121-c32242e63f39?q=80&w=1200",
        tags: &["Wi-Fi", "Breakfast", "Parking"],
        amenities: &["Free Wi-Fi", "Parking", "Restaurant", "24/7 Front Desk", "Laundry", "Air Conditioning"],
        description: "A long-running Hodan district hotel favoured by journalists and NGO staff. Simple, secure and reliably clean, with a generous breakfast included in every rate.",
        phone: "+252 61 222 3344", whatsapp: "+252 61 222 3344",
        photos: &[
            "https://images.unsplash.com/photo-1618773928121-c32242e63f39?q=80&w=1200",
            "https://images.unsplash.com/photo-1566665797739-1674de7a421a?q=80&w=1200",
        ],
        rooms: ROOMS_MID, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_GENERIC,
        check_in_time: "1:00 PM", check_out_time: "11:00 AM", featured: false,
        highlight: "Trusted by long-stay guests",
    },
    Hotel {
        id: "ambassador-hotel-hargeisa",
        name: "Ambassador Hotel Hargeisa",
        area: "26 June District", city: "Hargeisa", country: "Somaliland",
        rating: 4.3, review_count: 91, price_from: 2900, price_was: None, star_class: 4,
        image: "https://images.unsplash.com/photo-1596394516093-501ba68a0ba6?q=80&w=1200",
        tags: &["Wi-Fi", "Breakfast", "Parking", "Restaurant"],
        amenities: &["Free Wi-Fi", "Parking", "Restaurant", "24/7 Front Desk", "Gym", "Room Service", "Laundry", "Air Conditioning", "Family Rooms"],
        description: "Hargeisa's established business hotel, with a large garden restaurant and meeting facilities. Popular for conferences, so book early around the summer diaspora season.",
        phone: "+252 63 400 1122", whatsapp: "+252 63 400 1122",
        photos: &[
            "https://images.unsplash.com/photo-1596394516093-501ba68a0ba6?q=80&w=1200",
            "https://images.unsplash.com/photo-1590490360182-c33d57733427?q=80&w=1200",
            "https://images.unsplash.com/photo-1611892440504-42a792e24d32?q=80&w=1200",
        ],
        rooms: ROOMS_UPSCALE, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_GENERIC,
        check_in_time: "2:00 PM", check_out_time: "12:00 PM", featured: true,
        highlight: "Garden restaurant · Conference venue",
    },
    Hotel {
        id: "maansoor-hotel-hargeisa",
        name: "Maansoor Hotel",
        area: "Jigjiga Yar", city: "Hargeisa", country: "Somaliland",
        rating: 4.1, review_count: 64, price_from: 2200, price_was: None, star_class: 3,
        image: "https://images.unsplash.com/photo-1571003123894-1f0594d2b5d9?q=80&w=1200",
        tags: &["Wi-Fi", "Breakfast", "Parking"],
        amenities: &["Free Wi-Fi", "Parking", "Restaurant", "24/7 Front Desk", "Laundry"],
        description: "A quiet, walled compound hotel with an excellent kitchen and helpful staff. The garden seating is a genuine highlight in the evenings.",
        phone: "+252 63 411 2233", whatsapp: "+252 63 411 2233",
        photos: &[
            "https://images.unsplash.com/photo-1571003123894-1f0594d2b5d9?q=80&w=1200",
            "https://images.unsplash.com/photo-1631049307264-da0ec9d70304?q=80&w=1200",
        ],
        rooms: ROOMS_MID, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_GENERIC,
        check_in_time: "1:00 PM", check_out_time: "11:00 AM", featured: false,
        highlight: "Quiet compound with garden dining",
    },
    Hotel {
        id: "sarova-stanley-nairobi",
        name: "Sarova Stanley Nairobi",
        area: "CBD", city: "Nairobi", country: "Kenya",
        rating: 4.7, review_count: 386, price_from: 6800, price_was: Some(7900), star_class: 5,
        image: "https://images.unsplash.com/photo-1551882547-ff40c63fe5fa?q=80&w=1200",
        tags: &["Wi-Fi", "Pool", "Spa", "Breakfast"],
        amenities: &["Free Wi-Fi", "Restaurant", "Parking", "24/7 Front Desk", "Swimming Pool", "Gym", "Spa", "Bar", "Room Service", "Laundry", "Business Centre", "Air Conditioning", "Wheelchair Access"],
        description: "A Nairobi institution since 1902, in the heart of the CBD. The famous Thorn Tree Café sits at street level, there is a rooftop pool, and the rooms have been fully refurbished while keeping the building's colonial-era character.",
        phone: "+254 20 275 7000", whatsapp: "+254 70 275 7000",
        photos: &[
            "https://images.unsplash.com/photo-1551882547-ff40c63fe5fa?q=80&w=1200",
            "https://images.unsplash.com/photo-1591088398332-8a7791972843?q=80&w=1200",
            "https://images.unsplash.com/photo-1584132915807-fd1f5fbc078f?q=80&w=1200",
            "https://images.unsplash.com/photo-1631049035182-249067d7618e?q=80&w=1200",
        ],
        rooms: ROOMS_LUXURY, reviews: REVIEWS_GOLDEN_TULIP, landmarks: LANDMARKS_GENERIC,
        check_in_time: "2:00 PM", check_out_time: "11:00 AM", featured: true,
        highlight: "Historic landmark · Rooftop pool",
    },
    Hotel {
        id: "eka-hotel-nairobi",
        name: "Eka Hotel Nairobi",
        area: "Mombasa Road", city: "Nairobi", country: "Kenya",
        rating: 4.4, review_count: 172, price_from: 4400, price_was: None, star_class: 4,
        image: "https://images.unsplash.com/photo-1584132967334-10e028bd69f7?q=80&w=1200",
        tags: &["Wi-Fi", "Pool", "Gym", "Airport Shuttle"],
        amenities: &["Free Wi-Fi", "Airport Pickup", "Restaurant", "Parking", "24/7 Front Desk", "Swimming Pool", "Gym", "Bar", "Room Service", "Air Conditioning", "Business Centre"],
        description: "A modern business hotel on Mombasa Road, roughly halfway between the CBD and JKIA. The pool deck and gym are open late, which suits guests arriving on evening flights.",
        phone: "+254 20 366 0000", whatsapp: "+254 70 366 0000",
        photos: &[
            "https://images.unsplash.com/photo-1584132967334-10e028bd69f7?q=80&w=1200",
            "https://images.unsplash.com/photo-1595576508898-0ad5c879a061?q=80&w=1200",
            "https://images.unsplash.com/photo-1582719478250-c89cae4dc85b?q=80&w=1200",
        ],
        rooms: ROOMS_UPSCALE, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_GENERIC,
        check_in_time: "2:00 PM", check_out_time: "10:00 AM", featured: false,
        highlight: "Halfway between the CBD and JKIA",
    },
    Hotel {
        id: "samrat-hotel-dire-dawa",
        name: "Samrat Hotel Dire Dawa",
        area: "Kezira", city: "Dire Dawa", country: "Ethiopia",
        rating: 4.0, review_count: 38, price_from: 1600, price_was: None, star_class: 3,
        image: "https://images.unsplash.com/photo-1540541338287-41700207dee6?q=80&w=1200",
        tags: &["Wi-Fi", "Breakfast", "Parking"],
        amenities: &["Free Wi-Fi", "Parking", "Restaurant", "24/7 Front Desk", "Air Conditioning"],
        description: "A dependable base in Kezira, walking distance from the old railway station and the main market. Air conditioning in every room, which matters in Dire Dawa.",
        phone: "+251 25 111 2233", whatsapp: "+251 91 111 2233",
        photos: &[
            "https://images.unsplash.com/photo-1540541338287-41700207dee6?q=80&w=1200",
            "https://images.unsplash.com/photo-1560448204-e02f11c3d0e2?q=80&w=1200",
        ],
        rooms: ROOMS_MID, reviews: REVIEWS_GENERIC, landmarks: LANDMARKS_GENERIC,
        check_in_time: "12:00 PM", check_out_time: "11:00 AM", featured: false,
        highlight: "Walk to the historic railway station",
    },
    Hotel {
        id: "kempinski-djibouti",
        name: "Djibouti Palace Kempinski",
        area: "Marabout", city: "Djibouti City", country: "Djibouti",
        rating: 4.6, review_count: 154, price_from: 9800, price_was: Some(11500), star_class: 5,
        image: "https://images.unsplash.com/photo-1571896349842-33c89424de2d?q=80&w=1200",
        tags: &["Wi-Fi", "Pool", "Spa", "Beach"],
        amenities: &["Free Wi-Fi", "Airport Pickup", "Restaurant", "Parking", "24/7 Front Desk", "Swimming Pool", "Gym", "Spa", "Bar", "Room Service", "Laundry", "Business Centre", "Air Conditioning", "Family Rooms", "Wheelchair Access"],
        description: "A waterfront five-star resort on its own private beach and marina, with a large spa, several restaurants and gardens running down to the sea. The clear choice at the top of the Djibouti market.",
        phone: "+253 21 32 5555", whatsapp: "+253 77 32 5555",
        photos: &[
            "https://images.unsplash.com/photo-1571896349842-33c89424de2d?q=80&w=1200",
            "https://images.unsplash.com/photo-1584132915807-fd1f5fbc078f?q=80&w=1200",
            "https://images.unsplash.com/photo-1591088398332-8a7791972843?q=80&w=1200",
            "https://images.unsplash.com/photo-1631049035182-249067d7618e?q=80&w=1200",
        ],
        rooms: ROOMS_LUXURY, reviews: REVIEWS_GOLDEN_TULIP, landmarks: LANDMARKS_GENERIC,
        check_in_time: "3:00 PM", check_out_time: "12:00 PM", featured: true,
        highlight: "Private beach · Marina · Full spa",
    },
];

pub fn find_hotel(id: &str) -> Option<&'static Hotel> {
    HOTELS.iter().find(|h| h.id == id)
}

pub fn find_room(hotel: &'static Hotel, room_id: &str) -> Option<&'static RoomType> {
    hotel.rooms.iter().find(|r| r.id == room_id)
}

pub fn featured_hotels() -> Vec<&'static Hotel> {
    HOTELS.iter().filter(|h| h.featured).collect()
}

/// Hotels in the same city, excluding the one being viewed.
pub fn similar_hotels(hotel: &Hotel) -> Vec<&'static Hotel> {
    let mut out: Vec<&'static Hotel> = HOTELS
        .iter()
        .filter(|h| h.city == hotel.city && h.id != hotel.id)
        .collect();
    if out.len() < 3 {
        out.extend(
            HOTELS
                .iter()
                .filter(|h| h.city != hotel.city && h.id != hotel.id)
                .take(3 - out.len()),
        );
    }
    out.into_iter().take(3).collect()
}

/// Every distinct city, in the order hotels are declared.
pub fn all_cities() -> Vec<&'static str> {
    let mut seen: Vec<&'static str> = Vec::new();
    for h in HOTELS {
        if !seen.contains(&h.city) {
            seen.push(h.city);
        }
    }
    seen
}

/// Every distinct amenity across the catalogue, for the filter panel.
pub fn all_amenities() -> Vec<&'static str> {
    let mut seen: Vec<&'static str> = Vec::new();
    for h in HOTELS {
        for a in h.amenities {
            if !seen.contains(a) {
                seen.push(a);
            }
        }
    }
    seen.sort_unstable();
    seen
}

// ---------------------------------------------------------------------------
// Marketing content
// ---------------------------------------------------------------------------

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

pub const STATS: &[Stat] = &[
    Stat { value: "690+", label: "Verified hotels", icon: "building" },
    Stat { value: "6", label: "Countries covered", icon: "globe" },
    Stat { value: "48k+", label: "Nights reserved", icon: "calendar" },
    Stat { value: "4.6", label: "Average guest rating", icon: "star" },
];
