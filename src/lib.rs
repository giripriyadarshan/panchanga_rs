mod calculations;
pub mod panchanga;

#[cfg(test)]
fn panchanga() {
    use crate::panchanga::Panchanga;
    let panchanga = Panchanga::new(12, 12, 12, 12.0, 12.0);
}
