const DIRECTIONS: &[(i64, i64)] = &[
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const FLOWER: u8 = 42;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let number_of_rows = garden.len() as i64;

    for (y, row) in garden.iter().enumerate() {
        let mut row_output: Vec<char> = vec![];

        for (x, square) in row.bytes().enumerate() {
            if square == FLOWER {
                row_output.push('*');
                continue;
            }

            let mut adjacent_flower_count: u8 = 0;
            for (dx, dy) in DIRECTIONS {
                let row_size = row.len() as i64;
                let x2: i64 = x as i64 + *dx;
                let y2: i64 = y as i64 + *dy;

                if cell_is_inside_the_garden(x2, y2, number_of_rows, row_size) {
                    let adjacent_value = garden[y2 as usize].as_bytes()[x2 as usize];
                    if adjacent_value == FLOWER {
                        adjacent_flower_count += 1
                    }
                }
            }

            if adjacent_flower_count == 0 {
                row_output.push(' ');
            } else {
                row_output.push((adjacent_flower_count + 48u8) as char);
            }
        }
        result.push(String::from_iter(row_output))
    }

    result
}

fn cell_is_inside_the_garden(x: i64, y: i64, number_of_rows: i64, row_size: i64) -> bool {
    let outside_the_garden = x < 0 || x >= row_size || y < 0 || y >= number_of_rows;

    !outside_the_garden
}
