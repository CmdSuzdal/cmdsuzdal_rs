//! Structures used to represent and "infinite" hex board in the style of the
//! ones used in the [hive game](https://en.wikipedia.org/wiki/Hive_(game)).
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
//           s = -q -r
//
//     So a position in the hexboard is defined by the following trio:
//        (q, r, -q-r)

/// A cell inside an hexagons board.
///
/// The position of the cell inside the board is defined using the three coordinates
/// (q,r,s) as defined in the [Red Blob Games](https://www.redblobgames.com/grids/hexagons/implementation.html)
/// article.
#[derive(Default, Debug)]
pub struct HexCell {
    q: i32,
    r: i32,
    s: i32,
}

impl HexCell {
    pub fn new() -> HexCell {
        HexCell { q: 0, r: 0, s: 0 }
    }
}

