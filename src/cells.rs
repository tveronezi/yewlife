use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct Cell {
    line: i64,
    column: i64,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.column.cmp(&other.column),
        }
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) struct Cells {
    universe: BTreeSet<Cell>,
}

struct Boundaries {
    line_max: i64,
    line_min: i64,
    column_max: i64,
    column_min: i64,
}

impl Cells {
    fn boundaries(&self) -> Boundaries {
        let lines: BTreeSet<i64> = self.universe.iter().map(|cell| cell.line).collect();
        let line_max = *lines.iter().max().expect("empty collection?");
        let line_min = *lines.iter().min().expect("empty collection?");
        let column: BTreeSet<i64> = self.universe.iter().map(|cell| cell.column).collect();
        let column_max = *column.iter().max().expect("empty collection?");
        let column_min = *column.iter().min().expect("empty collection?");
        Boundaries {
            line_max,
            line_min,
            column_max,
            column_min,
        }
    }

    fn number_of_neighbors(&mut self, line: i64, column: i64) -> usize {
        vec![
            self.universe.get(&Cell {
                line: line - 1,
                column: column - 1,
            }), // top_left
            self.universe.get(&Cell {
                line: line - 1,
                column,
            }), // top
            self.universe.get(&Cell {
                line: line - 1,
                column: column + 1,
            }), // top_right
            self.universe.get(&Cell {
                line: line + 1,
                column: column - 1,
            }), // bottom_left
            self.universe.get(&Cell {
                line: line + 1,
                column,
            }), // bottom
            self.universe.get(&Cell {
                line: line + 1,
                column: column + 1,
            }), // bottom_right
            self.universe.get(&Cell {
                line,
                column: column - 1,
            }), // left
            self.universe.get(&Cell {
                line,
                column: column + 1,
            }), // right
        ]
        .iter()
        .map(|c| c.is_some())
        .filter(|c| *c)
        .collect::<Vec<bool>>()
        .len()
    }

    fn update(&mut self) {
        if self.universe.is_empty() {
            return;
        }
        let Boundaries {
            line_max: x_max,
            line_min: x_min,
            column_max: y_max,
            column_min: y_min,
        } = self.boundaries();
        let mut new_universe = BTreeSet::new();
        for line in x_min..x_max {
            for column in y_min..y_max {
                let neighbors = self.number_of_neighbors(line, column);
                let this_cell = Cell { line, column };
                let exist = self.universe.get(&this_cell).is_some();
                if !exist {
                    if neighbors == 3 {
                        new_universe.insert(this_cell);
                    }
                } else {
                    if neighbors == 3 || neighbors == 2 {
                        new_universe.insert(this_cell);
                    }
                }
            }
        }
        self.universe = new_universe;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_boundaries() {
        Cells {
            universe: BTreeSet::from_iter(
                vec![
                    Cell { line: 0, column: 1 },
                    Cell { line: 1, column: 2 },
                    Cell { line: 2, column: 0 },
                ]
                    .iter()
                    .cloned(),
            ),
        };
    }

    #[test]
    fn test_universe_update() {
        let universe = BTreeSet::from_iter(
            vec![
                Cell { line: 1, column: 0 },
                Cell { line: 0, column: 0 },
                Cell { line: 0, column: 0 },
                Cell { line: 0, column: 1 },
                Cell {
                    line: 0,
                    column: -1,
                },
            ]
            .iter()
            .cloned(),
        );
        // original universe
        // xx
        // x-
        let mut cells = Cells { universe };
        cells.update();
        // new universe
        // xx
        // xx
        assert_eq!(
            cells.universe.iter().cloned().collect::<Vec<Cell>>(),
            vec![
                Cell { line: 0, column: 0 },
                Cell { line: 0, column: 1 },
                Cell { line: 1, column: 0 },
                Cell { line: 1, column: 1 },
            ]
        );
    }

    #[test]
    fn test_universe_order() {
        let universe = BTreeSet::from_iter(
            vec![
                Cell { line: 1, column: 0 },
                Cell { line: 0, column: 0 },
                Cell { line: 0, column: 0 },
                Cell { line: 0, column: 1 },
                Cell {
                    line: 0,
                    column: -1,
                },
            ]
            .iter()
            .cloned(),
        );
        let cells = Cells { universe };
        let mut iter = cells.universe.iter();
        assert_eq!(
            Some(&Cell {
                line: 0,
                column: -1
            }),
            iter.next()
        );
        assert_eq!(Some(&Cell { line: 0, column: 0 }), iter.next());
        assert_eq!(Some(&Cell { line: 0, column: 1 }), iter.next());
        assert_eq!(Some(&Cell { line: 1, column: 0 }), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_cmp() {
        let cell_a = Cell { line: 0, column: 0 };
        let cell_b = Cell { line: 0, column: 0 };
        assert_eq!(Ordering::Equal, cell_a.cmp(&cell_b));
        let cell_a = Cell { line: 0, column: 0 };
        let cell_b = Cell { line: 1, column: 0 };
        assert_eq!(Ordering::Less, cell_a.cmp(&cell_b));
        let cell_a = Cell { line: 1, column: 0 };
        let cell_b = Cell { line: 0, column: 0 };
        assert_eq!(Ordering::Greater, cell_a.cmp(&cell_b));
        let cell_a = Cell { line: 0, column: 0 };
        let cell_b = Cell { line: 0, column: 1 };
        assert_eq!(Ordering::Less, cell_a.cmp(&cell_b));
        let cell_a = Cell { line: 0, column: 1 };
        let cell_b = Cell { line: 0, column: 0 };
        assert_eq!(Ordering::Greater, cell_a.cmp(&cell_b));
    }
}
