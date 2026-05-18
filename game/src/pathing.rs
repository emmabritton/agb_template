use agb::fixnum::Vector2D;

const MAP_WIDTH: usize = 15;
const MAP_HEIGHT: usize = 10;

const MAX_QUEUE_SIZE: usize = MAP_WIDTH * MAP_HEIGHT; // For 5x5 grid
const MAX_PATH_LENGTH: usize = MAX_QUEUE_SIZE;

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub struct Queue {
    data: [(usize, usize); MAX_QUEUE_SIZE],
    head: usize,
    tail: usize,
}

impl Queue {
    fn new() -> Self {
        Queue {
            data: [(0, 0); MAX_QUEUE_SIZE],
            head: 0,
            tail: 0,
        }
    }

    fn push(&mut self, x: usize, y: usize) {
        if self.tail < MAX_QUEUE_SIZE {
            self.data[self.tail] = (x, y);
            self.tail += 1;
        }
    }

    fn pop(&mut self) -> Option<(usize, usize)> {
        if self.head < self.tail {
            let item = self.data[self.head];
            self.head += 1;
            Some(item)
        } else {
            None
        }
    }
}

// BFS for GBA (using static arrays)
pub fn find_path_gba(
    map: &[u8],
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    path: &mut [(usize, usize); MAX_PATH_LENGTH],
) -> bool {
    let mut queue = Queue::new();
    let mut visited = [[false; MAP_HEIGHT]; MAP_WIDTH];
    let mut parent = [[(0, 0); MAP_HEIGHT]; MAP_WIDTH]; // Store parent coordinates

    queue.push(start_x, start_y);
    visited[start_y][start_x] = true;

    while let Some((x, y)) = queue.pop() {
        if x == goal_x && y == goal_y {
            // Reconstruct the path
            let mut current = (x, y);
            let mut length = 0;
            while current != (start_x, start_y) {
                path[length] = current;
                length += 1;
                current = parent[current.1][current.0];
            }
            path[length] = (start_x, start_y);
            return true;
        }

        for &(dx, dy) in &DIRECTIONS {
            let nx = x as i8 + dx;
            let ny = y as i8 + dy;

            if nx >= 0 && nx < MAP_WIDTH as i8 && ny >= 0 && ny < MAP_HEIGHT as i8 {
                let nx = nx as usize;
                let ny = ny as usize;
                if !visited[ny][nx] && map[ny * MAP_WIDTH + nx] == 0 {
                    visited[ny][nx] = true;
                    parent[ny][nx] = (x, y);
                    queue.push(nx, ny);
                }
            }
        }
    }

    false // No path found
}

// Output: A 2D array where `reachable[y][x]` = cost to reach (x,y), or 255 if unreachable
pub fn flood_fill(
    impassible: &[bool], // 1D map array (0 = floor, 1 = wall, etc.)
    start: Vector2D<i32>,
    max_movement: u8,
    reachable: &mut [[u8; MAP_HEIGHT]; MAP_WIDTH], // Output: cost to reach each tile
) {
    // Initialize reachable tiles (255 = unreachable)
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            reachable[x][y] = 255;
        }
    }

    // Queue for BFS: (x, y, cost_so_far)
    let mut queue = [(0, 0, 0); MAX_QUEUE_SIZE]; // Static array for GBA
    let mut queue_head = 0;
    let mut queue_tail = 0;

    // Start at the unit's position
    reachable[start.x as usize][start.y as usize] = 0;
    queue[queue_tail] = (start.x as u8, start.y as u8, 0);
    queue_tail += 1;

    while queue_head < queue_tail {
        let (x, y, cost) = queue[queue_head];
        queue_head += 1;

        // Skip if we've already found a better path to this tile
        if cost > reachable[x as usize][y as usize] {
            continue;
        }

        // Explore neighbors
        for &(dx, dy) in &DIRECTIONS {
            let nx = x as i8 + dx;
            let ny = y as i8 + dy;

            if nx >= 0 && nx < MAP_WIDTH as i8 && ny >= 0 && ny < MAP_HEIGHT as i8 {
                let nx = nx as usize;
                let ny = ny as usize;

                if impassible[ny * MAP_WIDTH + nx] {
                    continue;
                }

                let new_cost = cost;
                if new_cost <= max_movement && new_cost < reachable[nx][ny] {
                    reachable[nx][ny] = new_cost;
                    queue[queue_tail] = (nx as u8, ny as u8, new_cost);
                    queue_tail += 1;
                }
            }
        }
    }
}

pub fn reconstruct_path(
    reachable: &[[u8; MAP_HEIGHT]; MAP_WIDTH],
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    path: &mut [(usize, usize); MAX_PATH_LENGTH],
) -> usize {
    let mut length = 0;
    let mut x = goal_x;
    let mut y = goal_y;

    while (x, y) != (start_x, start_y) {
        path[length] = (x, y);
        length += 1;

        // Find the neighbor with the lowest cost
        let mut min_cost = 255;
        let mut next_x = x;
        let mut next_y = y;

        for &(dx, dy) in &DIRECTIONS {
            let nx = x as i8 + dx;
            let ny = y as i8 + dy;
            if nx >= 0 && nx < MAP_WIDTH as i8 && ny >= 0 && ny < MAP_HEIGHT as i8 {
                let nx = nx as usize;
                let ny = ny as usize;
                if reachable[nx][ny] < min_cost {
                    min_cost = reachable[nx][ny];
                    next_x = nx;
                    next_y = ny;
                }
            }
        }

        x = next_x;
        y = next_y;
    }

    path[length] = (start_x, start_y);
    length + 1
}
