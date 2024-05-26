mod calculations;
pub mod panchanga;

#[cfg(test)]
mod tests {
    use crate::panchanga::Panchanga;
    #[test]
    fn panchanga() {
        let panchanga = Panchanga::new(11, 4, 1998, 12.5, 5.5);
        assert_eq!("Poornima".to_string(), panchanga.tithi);
        assert_eq!("Hasta".to_string(), panchanga.nakshatra);
        assert_eq!("Vyaghata".to_string(), panchanga.yoga);
        assert_eq!("Visti".to_string(), panchanga.karan);
        assert_eq!("Kanya".to_string(), panchanga.rashi);
        assert_eq!("Shukla".to_string(), panchanga.paksha);
    }
}
