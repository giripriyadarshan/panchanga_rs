use crate::calculations::{calculate_ayanamasa, rev, Longitudes};

pub struct Panchanga {
    pub rashi: String,
    pub tithi: String,
    pub karan: String,
    pub paksha: String,
    pub yoga: String,
    pub nakshatra: String,
}

impl Panchanga {
    pub fn new(dd: i64, mm: i64, yy: i64, hr: f64, zhr: f64) -> Self {
        let rashi_list = [
            "Mesha",
            "Vrishabha",
            "Mithuna",
            "Karka",
            "Simha",
            "Kanya",
            "Tula",
            "Vrischika",
            "Dhanu",
            "Makara",
            "Kumbha",
            "Meena",
        ];

        let tithi_list = [
            "Prathame",
            "Dwithiya",
            "Thrithiya",
            "Chathurthi",
            "Panchami",
            "Shrashti",
            "Saptami",
            "Ashtami",
            "Navami",
            "Dashami",
            "Ekadashi",
            "Dwadashi",
            "Thrayodashi",
            "Chaturdashi",
            "Poornima/Amavasya",
        ];

        let karan_list = [
            "Bava",
            "Balava",
            "Kaulava",
            "Taitula",
            "Garija",
            "Vanija",
            "Visti",
            "Sakuni",
            "Chatuspada",
            "Naga",
            "Kimstughna",
        ];

        let yoga_list = [
            "Vishkambha",
            "Prithi",
            "Ayushman",
            "Saubhagya",
            "Shobhana",
            "Atiganda",
            "Sukarman",
            "Dhrithi",
            "Shoola",
            "Ganda",
            "Vridhi",
            "Dhruva",
            "Vyaghata",
            "Harshana",
            "Vajra",
            "Siddhi",
            "Vyatipata",
            "Variyan",
            "Parigha",
            "Shiva",
            "Siddha",
            "Sadhya",
            "Shubha",
            "Shukla",
            "Bramha",
            "Indra",
            "Vaidhruthi",
        ];

        let nakshatra_list = [
            "Ashwini",
            "Bharani",
            "Krittika",
            "Rohini",
            "Mrigashira",
            "Ardhra",
            "Punarvasu",
            "Pushya",
            "Ashlesa",
            "Magha",
            "Poorva Phalguni",
            "Uttara Phalguni",
            "Hasta",
            "Chitra",
            "Swathi",
            "Vishaka",
            "Anuradha",
            "Jyeshta",
            "Mula",
            "Poorva Ashada",
            "Uttara Ashada",
            "Sravana",
            "Dhanishta",
            "Shatabisha",
            "Poorva Bhadra",
            "Uttara Bhadra",
            "Revathi",
        ];

        let d = (367 * yy - 7 * (yy + (mm + 9) / 12) / 4 + 275 * mm / 9 + dd - 730530) as f64;

        let ayanamasa = calculate_ayanamasa(d);

        let longitudes = Longitudes::get(d + ((hr - zhr) / 24.0));
        let (slon, mlon) = (longitudes.sun, longitudes.moon);

        //Calculate Tithi and Paksha
        let tmlon = if mlon < slon { mlon + 360_f64 } else { mlon };
        let tslon = slon;
        let n: usize = ((tmlon - tslon) / 12_f64) as usize;

        //TODO: Rewrite it to match poornima and amavasya logic
        let tithi = tithi_list[n].to_string();
        let paksha = if n <= 14 {
            "Shukla".to_string()
        } else {
            "Krishna".to_string()
        };

        //Calculate Karana
        let mut n = ((tmlon - tslon) / 6.0) as usize;
        if n == 0 {
            n = 10
        }
        if n >= 57 {
            n -= 50
        }
        if n > 0 && n < 57 {
            n = (n - 1) - ((n - 1) / 7 * 7)
        }

        let karan = karan_list[n].to_string();

        //Calculate Nakshatra
        let tmlon = rev(mlon + ayanamasa);
        let nakshatra = nakshatra_list[(tmlon * 6.0 / 80.0) as usize].to_string();

        //Calculate Yoga
        let tmlon = mlon + ayanamasa;
        let tslon = slon + ayanamasa;
        let yoga = yoga_list[(rev(tmlon + tslon) * 6.0 / 80.0) as usize].to_string();

        //Calculate the rashi in which the moon is present
        let tmlon = rev(mlon + ayanamasa);
        let rashi = rashi_list[(tmlon / 30.0) as usize].to_string();

        Panchanga {
            rashi,
            tithi,
            karan,
            yoga,
            nakshatra,
            paksha,
        }
    }
}
