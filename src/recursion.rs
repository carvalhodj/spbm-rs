use std::i32::MAX;

fn is_safe(maze: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, x: i32, y: i32) -> bool {
    x >= 0
        && (x as usize) < maze.len()
        && y >= 0
        && (y as usize) < maze[0].len()
        && maze[x as usize][y as usize] == 1
        && !visited[x as usize][y as usize]
}

fn find_shortest_path<'a>(
    maze: &Vec<Vec<i32>>,
    visited: &'a mut Vec<Vec<bool>>,
    i: i32,
    j: i32,
    x: i32,
    y: i32,
    min_dist: &'a mut i32,
    dist: &'a mut i32,
) {
    if i == x && j == y {
        *min_dist = std::cmp::min(*min_dist, *dist);
        return;
    }

    visited[i as usize][j as usize] = true;

    if is_safe(maze, visited, i + 1, j) {
        *dist = *dist + 1;
        find_shortest_path(maze, visited, i + 1, j, x, y, min_dist, dist);
        *dist = *dist - 1;
    }
    if is_safe(maze, visited, i, j + 1) {
        *dist = *dist + 1;
        find_shortest_path(maze, visited, i, j + 1, x, y, min_dist, dist);
        *dist = *dist - 1;
    }
    if is_safe(maze, visited, i - 1, j) {
        *dist = *dist + 1;
        find_shortest_path(maze, visited, i - 1, j, x, y, min_dist, dist);
        *dist = *dist - 1;
    }
    if is_safe(maze, visited, i, j - 1) {
        *dist = *dist + 1;
        find_shortest_path(maze, visited, i, j - 1, x, y, min_dist, dist);
        *dist = *dist - 1;
    }

    visited[i as usize][j as usize] = false;
}

pub fn shortest_path_binary_matrix(maze: &Vec<Vec<i32>>, src: (i32, i32), dst: (i32, i32)) -> i32 {
    if maze.is_empty()
        || maze[src.0 as usize][src.1 as usize] == 0
        || maze[dst.0 as usize][dst.1 as usize] == 0
    {
        return -1;
    }

    let row = maze.len();
    let col = maze[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; col]; row];
    let mut dist = MAX;
    let mut tmp = 0;

    find_shortest_path(
        &maze,
        &mut visited,
        src.0,
        src.1,
        dst.0,
        dst.1,
        &mut dist,
        &mut tmp,
    );

    if dist != MAX {
        return dist;
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let maze = vec![
            vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 0],
            vec![1, 0, 1, 1, 1, 1, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 1, 0, 1, 1, 1],
            vec![1, 1, 0, 0, 0, 0, 1, 0, 0, 1],
        ];

        let src = (0, 0);
        let dst = (0, 9);
        let dist = shortest_path_binary_matrix(&maze, src, dst);
        assert_eq!(dist, 13);
    }
}
