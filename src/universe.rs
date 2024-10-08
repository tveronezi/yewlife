use crate::components::Dimensions;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;
use yew::prelude::*;

pub const CELL_SIZE: i32 = 20;

#[derive(Eq, PartialEq, Debug, Clone, Properties)]
pub struct Entity {
    pub line: i32,
    pub column: i32,
}

impl Entity {
    pub fn is_visible(&self, dimensions: &Dimensions) -> bool {
        if self.column < 0 || self.line < 0 {
            return false;
        }
        let outside_width = self.column * crate::universe::CELL_SIZE > dimensions.width;
        let outside_height = self.line * crate::universe::CELL_SIZE > dimensions.height;
        !(outside_height || outside_width)
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.column.cmp(&other.column),
        }
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Properties)]
pub struct Universe {
    pub entities: BTreeSet<Rc<Entity>>,
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Universe {:?}",
            self.entities.iter().cloned().collect::<Vec<Rc<Entity>>>()
        )
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Properties)]
struct Boundaries {
    line_max: i32,
    line_min: i32,
    column_max: i32,
    column_min: i32,
}

impl Universe {
    pub fn new(matrix: &str) -> Self {
        let mut universe = BTreeSet::new();
        for (line, line_value) in matrix.trim().lines().enumerate() {
            for (column, entry) in line_value.chars().enumerate() {
                if entry == '1' {
                    universe.insert(Rc::new(Entity {
                        line: line as i32,
                        column: column as i32,
                    }));
                }
            }
        }
        Universe { entities: universe }
    }

    fn boundaries(&self) -> Boundaries {
        let lines: BTreeSet<i32> = self.entities.iter().map(|cell| cell.line).collect();
        let line_max = *lines.iter().max().expect("empty collection?");
        let line_min = *lines.iter().min().expect("empty collection?");
        let column: BTreeSet<i32> = self.entities.iter().map(|cell| cell.column).collect();
        let column_max = *column.iter().max().expect("empty collection?");
        let column_min = *column.iter().min().expect("empty collection?");
        Boundaries {
            line_max,
            line_min,
            column_max,
            column_min,
        }
    }

    fn number_of_neighbors(&mut self, line: i32, column: i32) -> usize {
        [
            self.entities.get(&Entity {
                line: line - 1,
                column: column - 1,
            }), // top_left
            self.entities.get(&Entity {
                line: line - 1,
                column,
            }), // top
            self.entities.get(&Entity {
                line: line - 1,
                column: column + 1,
            }), // top_right
            self.entities.get(&Entity {
                line: line + 1,
                column: column - 1,
            }), // bottom_left
            self.entities.get(&Entity {
                line: line + 1,
                column,
            }), // bottom
            self.entities.get(&Entity {
                line: line + 1,
                column: column + 1,
            }), // bottom_right
            self.entities.get(&Entity {
                line,
                column: column - 1,
            }), // left
            self.entities.get(&Entity {
                line,
                column: column + 1,
            }), // right
        ]
        .iter()
        .map(|c| c.is_some())
        .filter(|c| *c)
        .count()
    }

    pub fn tick(&mut self) {
        if self.entities.is_empty() {
            return;
        }
        let Boundaries {
            line_max: x_max,
            line_min: x_min,
            column_max: y_max,
            column_min: y_min,
        } = self.boundaries();
        let mut new_entities = BTreeSet::new();
        for line in (x_min - 1)..(x_max + 2) {
            for column in (y_min - 1)..(y_max + 2) {
                let neighbors = self.number_of_neighbors(line, column);
                let this_cell = Rc::new(Entity { line, column });
                let exist = self.entities.contains(&this_cell);
                if !exist {
                    if neighbors == 3 {
                        new_entities.insert(this_cell);
                    }
                } else if neighbors == 3 || neighbors == 2 {
                    new_entities.insert(this_cell);
                }
            }
        }
        self.entities = new_entities;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_universe_update() {
        let mut universe = Universe::new(
            r#"
010
001
111
        "#,
        );
        universe.tick();
        assert_eq!(
            Universe::new(
                r#"
000
101
011
010
        "#
            ),
            universe
        );
        universe.tick();
        assert_eq!(
            Universe::new(
                r#"
000
001
101
011
        "#
            ),
            universe
        );
        universe.tick();
        assert_eq!(
            Universe::new(
                r#"
0000
0100
0011
0110
        "#
            ),
            universe
        );
        universe.tick();
        assert_eq!(
            Universe::new(
                r#"
0000
0010
0001
0111
        "#
            ),
            universe
        );
    }

    #[test]
    fn test_universe_order() {
        let entities = BTreeSet::from_iter(
            [
                Rc::new(Entity { line: 1, column: 0 }),
                Rc::new(Entity { line: 0, column: 0 }),
                Rc::new(Entity { line: 0, column: 0 }),
                Rc::new(Entity { line: 0, column: 1 }),
                Rc::new(Entity {
                    line: 0,
                    column: -1,
                }),
            ]
            .iter()
            .cloned(),
        );
        let universe = Universe { entities };
        let mut iter = universe.entities.iter();
        assert_eq!(
            Some(&Rc::new(Entity {
                line: 0,
                column: -1
            })),
            iter.next()
        );
        assert_eq!(Some(&Rc::new(Entity { line: 0, column: 0 })), iter.next());
        assert_eq!(Some(&Rc::new(Entity { line: 0, column: 1 })), iter.next());
        assert_eq!(Some(&Rc::new(Entity { line: 1, column: 0 })), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_cmp() {
        let cell_a = Entity { line: 0, column: 0 };
        let cell_b = Entity { line: 0, column: 0 };
        assert_eq!(Ordering::Equal, cell_a.cmp(&cell_b));
        let cell_a = Entity { line: 0, column: 0 };
        let cell_b = Entity { line: 1, column: 0 };
        assert_eq!(Ordering::Less, cell_a.cmp(&cell_b));
        let cell_a = Entity { line: 1, column: 0 };
        let cell_b = Entity { line: 0, column: 0 };
        assert_eq!(Ordering::Greater, cell_a.cmp(&cell_b));
        let cell_a = Entity { line: 0, column: 0 };
        let cell_b = Entity { line: 0, column: 1 };
        assert_eq!(Ordering::Less, cell_a.cmp(&cell_b));
        let cell_a = Entity { line: 0, column: 1 };
        let cell_b = Entity { line: 0, column: 0 };
        assert_eq!(Ordering::Greater, cell_a.cmp(&cell_b));
    }
}
