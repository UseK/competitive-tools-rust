use crate::io::parse_line;

pub struct Maze<T> {
    rows: usize,
    columns: usize,
    field: Vec<Vec<T>>,
}

impl Maze<bool> {
    pub fn dfs(&mut self, x: usize, y: usize) {
        if self.field[y][x] {
            self.field[y][x] = false;
            for (moved_x, moved_y) in self.movable_positions(x, y) {
                self.dfs(moved_x, moved_y);
            }
        }
    }

    pub fn movable_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut positions = vec![];
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                };
                if (x == 0 && dx == -1) || (y == 0 && dy == -1) {
                    continue;
                };
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;
                if (self.columns <= new_x) || (self.rows <= new_y) {
                    continue;
                };
                positions.push((new_x, new_y));
            }
        }
        positions
    }
}

pub fn parse_maze(h: usize, wall_ch: char) -> Vec<Vec<bool>> {
    (0..h)
        .map(|_| {
            parse_line::<String>()
                .chars()
                .map(|s| s != wall_ch)
                .collect()
        })
        .collect()
}

/// ```
/// use competitive_tools_rust::maze::surround_wall;
/// let mut maze = vec![vec![true]];
/// surround_wall(&mut maze);
/// assert_eq!(maze, vec![
///     vec![false, false, false],
///     vec![false, true, false],
///     vec![false, false, false],
/// ])
///
/// ```
pub fn surround_wall(maze: &mut Vec<Vec<bool>>) {
    maze.iter_mut().for_each(|line| {
        line.insert(0, false);
        line.push(false);
    });
    let n_line = maze[0].len();
    maze.insert(0, vec![false; n_line]);
    maze.push(vec![false; n_line]);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests_maze {
    //use crate::d;
    use crate::maze::Maze;

    #[test]
    fn test_poj2386() {
        let n = 10;
        let m = 12;
        let input_lines = vec![
            "W........WW.".to_string(),
            ".WWW.....WWW".to_string(),
            "....WW...WW.".to_string(),
            ".........WW.".to_string(),
            ".........W..".to_string(),
            "..W......W..".to_string(),
            ".W.W.....WW.".to_string(),
            "W.W.W.....W.".to_string(),
            ".W.W......W.".to_string(),
            "..W.......W.".to_string(),
        ];
        //d!(input_lines);
        let field: Vec<Vec<bool>> = input_lines
            .into_iter()
            .map(|line| line.chars().map(|c| c == 'W').collect())
            .collect();
        let mut maze: Maze<bool> = Maze {
            rows: n,
            columns: m,
            field,
        };
        let mut ans = 0;
        for x in 0..m {
            for y in 0..n {
                if maze.field[y][x] {
                    maze.dfs(x, y);
                    ans += 1;
                }
            }
        }
        for line in maze.field.iter() {
            assert!(line.iter().all(|&i| !i));
        }
        assert_eq!(ans, 3);
    }
}
