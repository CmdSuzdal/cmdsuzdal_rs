//! Structures used to represent and "infinite" hex board in the style of the
//! ones used for example in the [hive game](https://en.wikipedia.org/wiki/Hive_(game)).
//!
//! Based upon the work on Red Blog Games, especially the
//! [base article](https://www.redblobgames.com/grids/hexagons/) and the
//! [article with implementation guidelines](https://www.redblobgames.com/grids/hexagons/implementation.html)

// ------------- Coordinate system -------------
//
//       -r
//   +s  |   +q
//    \  |  /
//     \ | /
//       o
//     / | \
//    /  |  \
//  -q   |   -s
//       +r
//   __________q(x)_________    __________r(z)__________   __________s(y)__________
//   |-1 | 0 | 1 | 2 | 3 |      |-3 |-3 |-3 |-3 |-3 |      |   | 3 | 2 | 1 | 0 |
//    \ / \ / \ / \ / \ / \      \ / \ / \ / \ / \ / \      \ / \ / \ / \ / \ / \
//     |-1 | 0 | 1 | 2 | 3 |      |-2 |-2 |-2 |-2 |-2 |      | 3 | 2 | 1 | 0 |-1 |
//    / \ / \ / \ / \ / \ /      / \ / \ / \ / \ / \ /      / \ / \ / \ / \ / \ /
//   |-2 |-1 | 0 | 1 | 2 |      |-1 |-1 |-1 |-1 |-1 |      | 3 | 2 | 1 | 0 |-1 |
//    \ / \ / \ / \ / \ / \      \ / \ / \ / \ / \ / \      \ / \ / \ / \ / \ / \
//     |-2 |-1 | 0 | 1 | 2 |      | 0 | 0 | 0 | 0 | 0 |      | 2 | 1 | 0 |-1 |-2 |
//    / \ / \ / \ / \ / \ /      / \ / \ / \ / \ / \ /      / \ / \ / \ / \ / \ /
//   |-3 |-2 |-1 | 0 | 1 |      | 1 | 1 | 1 | 1 | 1 |      | 2 | 1 | 0 |-1 |-2 |
//    \ / \ / \ / \ / \ / \      \ / \ / \ / \ / \ / \      \ / \ / \ / \ / \ / \
//     |-3 |-2 |-1 | 0 | 1 |      | 2 | 2 | 2 | 2 | 2 |      | 1 | 0 |-1 |-2 |-3 |
//    / \ / \ / \ / \ / \ /      / \ / \ / \ / \ / \ /      / \ / \ / \ / \ / \ /
//   |-4 |-3 |-2 |-1 | 0 |      | 3 | 3 | 3 | 3 | 3 |      | 1 | 0 |-1 |-2 |-3 |
//
//     For each cell: q + r + s = 0
//
//     For this reason two coordinates are sufficient to identify a cell.
//     Normally q and r are used and s is computed:
//           s = -q-r
//
//     So a position in the hexboard is defined by the following trio:
//        (q, r, -q-r)

/// A cell inside an hexagons board.
///
/// The position of the cell inside the board is defined using the three coordinates
/// (q,r,s) as defined in the [Red Blob Games](https://www.redblobgames.com/grids/hexagons/implementation.html)
/// article.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct HexCell {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexCell {
    /// Default constructor for the [HexCell] struct: instantiate an [HexCell]
    /// at origin of field (coordinates 0,0,0).
    pub fn new() -> HexCell {
        HexCell { q: 0, r: 0, s: 0 }
    }

    /// Set an [HexCell] to position (q,r, -q-r)
    ///
    /// The [HexCell] position is defined by a trio of coordinates
    /// (q,r,s) of which only the first two (q,r) are free, since the
    /// third (s) can be computed with the formula: s = -q-r
    ///
    /// # Example
    /// ```
    /// # use abbadingo::hexboard::HexCell;
    /// let mut xc = HexCell::new();
    /// xc.set(3, -1);
    /// assert_eq!(xc, HexCell {q: 3, r: -1, s: -2});
    /// ```
    pub fn set(&mut self, q: i32, r: i32)
    {
        self.q = q;
        self.r = r;
        self.s = -q -r;
    }
}

// ****************************************************************************
// TESTS
// ****************************************************************************
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn by_default_a_new_hexcell_is_at_zero_coordinates() {
        let xc = HexCell::new();
        assert_eq!((xc.q, xc.r, xc.s), (0, 0, 0));
    }

}
