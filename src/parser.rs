use std::fs;

pub struct Grid {
    pub cells: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub start: (usize, usize),
}

pub fn parse(filename: &str) -> Result<Grid, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let lines: Vec<&str> = content.lines().collect();

    let height = lines.len();
    if height == 0 {
        return Err("Empty file".into());
    }

    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    if width == 0 {
        return Err("Empty grid".into());
    }

    let mut cells = vec![vec!['.'; width]; height];
    let mut start = None;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            cells[y][x] = ch;
            if ch == 'S' {
                start = Some((x, y));
            }
        }
    }

    let start = start.unwrap_or((0, 0));

    Ok(Grid {
        cells,
        width,
        height,
        start,
    })
}
