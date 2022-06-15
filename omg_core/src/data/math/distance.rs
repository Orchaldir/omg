/// Returns distance between 2 points in 2d space.
///
/// ```
///# use omg_core::data::math::distance::calculate_distance;
///
/// assert_eq!(calculate_distance(0, 0, 0, 0), 0);
/// assert_eq!(calculate_distance(0, 0, 3, 4), 5);
/// assert_eq!(calculate_distance(3, 4, 0, 0), 5);
/// ```
///
/// # Panics
///
/// Panics if the distance is greater than the possible maximum.
///
/// ```should_panic
///# use omg_core::data::math::distance::calculate_distance;
///
/// calculate_distance(0, 0, u32::MAX, u32::MAX);
/// ```
pub fn calculate_distance(x0: u32, y0: u32, x1: u32, y1: u32) -> u32 {
    let diff_x = x0.abs_diff(x1);
    let diff_y = y0.abs_diff(y1);
    let squarred_distance = diff_x.pow(2) + diff_y.pow(2);
    (squarred_distance as f64).sqrt() as u32
}
