use crate::data::math::size2d::Size2d;
use anyhow::{bail, Result};

#[svgbobdoc::transform]
/// A 2 dimensional lookup table with cells of equal size.
///
/// # Diagram
///
/// ```svgbob
///      input 1
///        ^
///        |
///        |----*----*----*----*
///        | 99 |  6 | 55 | 77 |
///        |----*----*----*----*
///        | 34 | 21 |  3 | 12 |
///        |----*----*----*----*
///        | 44 |  1 |  1 | 88 |
///        +----*----*----*----*--> input 0
/// ```
///
/// Each cell has a user defined value.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LookupTable2d {
    size: Size2d,
    cell_size: Size2d,
    values: Vec<u8>,
}

impl LookupTable2d {
    /// Returns a the lookup table, if valid:
    ///
    /// ```
    ///# use omg_core::data::math::size2d::Size2d;
    ///# use omg_core::data::math::transformer::lookup2d::LookupTable2d;
    /// assert!(LookupTable2d::new(Size2d::unchecked(2,  2), vec![10, 20]).is_err());
    /// assert!(LookupTable2d::new(Size2d::unchecked(0,  0), vec![10, 20]).is_err());
    /// assert!(LookupTable2d::new(Size2d::unchecked(0,  0), vec![]).is_err());
    /// ```
    pub fn new(size: Size2d, values: Vec<u8>) -> Result<LookupTable2d> {
        if size.get_area() != values.len() {
            bail!(
                "The size of the lookup table ({}) doesn't match the number of values ({})!",
                size.get_area(),
                values.len()
            );
        } else if values.len() < 2 {
            bail!("The lookup table has too few values!");
        }

        let width = calculate_cell_size(size.width());
        let height = calculate_cell_size(size.height());

        Ok(LookupTable2d {
            size,
            cell_size: Size2d::new(width, height).expect("Cell size is invalid"),
            values,
        })
    }

    pub fn size(&self) -> &Size2d {
        &self.size
    }

    pub fn values(&self) -> &[u8] {
        &self.values
    }

    /// Returns the value of the cell for the input values.
    ///
    /// ```
    ///# use omg_core::data::math::size2d::Size2d;
    ///# use omg_core::data::math::transformer::lookup2d::LookupTable2d;
    /// let table = LookupTable2d::new(Size2d::unchecked(3, 2), vec![10, 20, 30, 40, 50, 60]).unwrap();
    ///
    /// assert_eq!(table.lookup(  0,   0), 10);
    /// assert_eq!(table.lookup(100,  60), 20);
    /// assert_eq!(table.lookup(200, 100), 30);
    /// assert_eq!(table.lookup( 60, 170), 40);
    /// assert_eq!(table.lookup(170, 200), 50);
    /// assert_eq!(table.lookup(255, 255), 60);
    /// ```
    pub fn lookup(&self, input0: u8, input1: u8) -> u8 {
        let x = input0 as u32 / self.cell_size.width();
        let y = input1 as u32 / self.cell_size.height();
        let index = self.size.to_index_risky(x, y);

        self.values[index]
    }
}

fn calculate_cell_size(number_of_cells: u32) -> u32 {
    (256.0 / number_of_cells as f32).ceil() as u32
}
