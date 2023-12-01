use crate::runner::run;
use itertools::Itertools;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
    value: u32,
}

struct Grid {
    points: Vec<Vec<Point>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn new(input: &String) -> Grid {
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;
        let points = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        let value = match char {
                            'S' => {
                                start = Some((x, y));
                                'a' as u32
                            }
                            'E' => {
                                end = Some((x, y));
                                'z' as u32
                            }
                            _ => char as u32,
                        };
                        Point { x, y, value }
                    })
                    .collect_vec()
            })
            .collect_vec();
        Grid {
            points,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn get_neighbors(&self, point: &Point) -> Vec<&Point> {
        let mut neighbors = Vec::new();
        let x = point.x;
        let y = point.y;

        if y > 0 {
            neighbors.push(&self.points[y - 1][x]);
        }
        if x > 0 {
            neighbors.push(&self.points[y][x - 1]);
        }
        if x < (self.points[y].len() - 1) {
            neighbors.push(&self.points[y][x + 1]);
        }
        if y < (self.points.len() - 1) {
            neighbors.push(&self.points[y + 1][x]);
        }

        neighbors
            .iter()
            .filter(|target| target.value <= (point.value + 1))
            .map(|p| *p)
            .collect_vec()
    }
}

fn part_1(input: &String) -> u32 {
    let grid = Grid::new(input);
    let end = &grid.points[grid.end.1][grid.end.0];

    let path = cam_star(
        &grid.points[grid.start.1][grid.start.0],
        |point| {
            grid.get_neighbors(point)
                .into_iter()
                .map(|p| (p.clone(), 1))
                .collect_vec()
        },
        |point| {
            let delta_x = (end.x as i32 - point.x as i32).abs();
            let delta_y = (end.y as i32 - point.y as i32).abs();
            (delta_x + delta_y) as u32
        },
        |point| point == end,
    );

    path.unwrap().1
}

fn part_2(input: &String) -> u32 {
    let grid = Grid::new(input);
    let end = &grid.points[grid.end.1][grid.end.0];
    let mut paths = Vec::new();
    let viable_starts = grid
        .points
        .iter()
        .flat_map(|row| row.iter())
        .filter(|point| point.value == 'a' as u32)
        .filter(|point| {
            grid.get_neighbors(point)
                .iter()
                .any(|neighbor| neighbor.value == 'b' as u32)
        })
        .collect_vec();

    for start in viable_starts {
        let path = cam_star(
            &grid.points[start.y][start.x],
            |point| {
                grid.get_neighbors(point)
                    .into_iter()
                    .map(|p| (p.clone(), 1))
                    .collect_vec()
            },
            |point| {
                let delta_x = (end.x as i32 - point.x as i32).abs();
                let delta_y = (end.y as i32 - point.y as i32).abs();
                (delta_x + delta_y) as u32
            },
            |point| point == end,
        );

        paths.push(path.unwrap());
    }

    paths.sort_by(|a, b| a.1.cmp(&b.1));

    paths.first().unwrap().1
}

// Cam* algorithm, one of the slowest pathfinding algorithms I've ever seen. Seems to work though.
fn cam_star<N, H, S>(
    start: &Point,
    mut neighbors: N,
    mut heuristic: H,
    mut success_condition: S,
) -> Option<(Vec<Point>, u32)>
where
    N: FnMut(&Point) -> Vec<(Point, u32)>,
    H: FnMut(&Point) -> u32,
    S: FnMut(&Point) -> bool,
{
    let mut closed_list: Vec<APoint> = Vec::new();
    let mut open_list: Vec<APoint> = Vec::new();

    let first_point = APoint {
        point: start.clone(),
        parent: None,
        g: 0,
        f: heuristic(start),
    };

    open_list.push(first_point.clone());

    while !open_list.is_empty() {
        // Much debugging was required to figure out that this should be sorted by g, not f
        open_list.sort_by(|a, b| b.g.cmp(&a.g));

        let current_point = open_list.pop().unwrap();
        closed_list.push(current_point.clone());

        if success_condition(&current_point.point.clone()) {
            let mut last = closed_list.last().unwrap();
            let mut path = vec![];
            while last.parent.is_some() {
                let parent = last.parent.as_ref().unwrap();
                path.push(parent.point.clone());
                last = parent;
            }
            let len = path.len() as u32;
            return Some((path, len));
        }

        for neighbor in neighbors(&current_point.point) {
            if closed_list.iter().any(|i| i.point == neighbor.0) {
                continue;
            }

            let open_list_point = open_list.iter_mut().find(|i| i.point == neighbor.0);
            if open_list_point.is_some() {
                let existing_point = open_list_point.unwrap();
                let new_point = APoint::from(&neighbor.0, &current_point, heuristic(&neighbor.0));
                if new_point.f < new_point.f {
                    existing_point.f = new_point.f;
                    existing_point.parent = new_point.parent;
                }
            } else {
                open_list.push(APoint::from(
                    &neighbor.0,
                    &current_point,
                    heuristic(&neighbor.0),
                ));
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct APoint {
    point: Point,
    parent: Option<Box<APoint>>,
    g: u32,
    f: u32,
}

impl APoint {
    fn from(point: &Point, parent: &APoint, h: u32) -> APoint {
        let g = parent.g + 1;

        let a_point = APoint {
            point: point.clone(),
            parent: Some(Box::new(parent.clone())),
            g,
            f: g + h,
        };
        a_point
    }
}
