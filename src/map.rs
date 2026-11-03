//! `map` — deterministic procedural maps for game design artifacts.
//!
//! Three variants, all seeded by the spec's `seed` (SplitMix64, same stream
//! contract as every other kind — the same spec renders the same world, byte
//! for byte, forever):
//!
//! - `terrain` — smooth value noise (2 octaves, bilinear) classified into
//!   five biome bands (`≈ ░ ▒ ▓ ▲`);
//! - `dungeon` — non-overlapping rectangular rooms joined by L corridors;
//! - `maze` — perfect maze (recursive backtracker): every cell reachable,
//!   exactly one path between any two.
//!
//! The body ends with a legend whose counts are computed from the rendered
//! grid, so the picture and its own legend cannot disagree.

use crate::rng::Rng;
use serde_json::Value;

/// Render a map. Public entry from the render dispatcher.
pub fn render_map(spec: &Value) -> Result<String, String> {
    let variant = spec
        .get("variant")
        .and_then(|v| v.as_str())
        .unwrap_or("terrain");
    let mut out: Vec<String> = Vec::new();
    if let Some(t) = crate::render::truthy_title(spec) {
        out.push(format!("# {t}"));
        out.push(String::new());
    }
    match variant {
        "terrain" => terrain(spec, &mut out)?,
        "dungeon" => dungeon(spec, &mut out)?,
        "maze" => maze(spec, &mut out)?,
        other => {
            return Err(format!(
                "map.variant must be terrain|dungeon|maze, got {other:?}"
            ))
        }
    }
    out.push(String::new());
    Ok(out.join("\n"))
}

fn dims(spec: &Value, dw: usize, dh: usize) -> Result<(usize, usize), String> {
    let get = |k: &str, d: usize| -> Result<usize, String> {
        match spec.get(k) {
            None | Some(Value::Null) => Ok(d),
            Some(v) => v
                .as_u64()
                .map(|n| n as usize)
                .ok_or_else(|| format!("map.{k} must be a positive int")),
        }
    };
    let w = get("w", dw)?.clamp(5, 160);
    let h = get("h", dh)?.clamp(5, 120);
    Ok((w, h))
}

fn push_grid(out: &mut Vec<String>, grid: &[Vec<char>]) {
    for row in grid {
        out.push(row.iter().collect());
    }
}

/// Count glyphs and append the legend lines.
fn push_legend(out: &mut Vec<String>, grid: &[Vec<char>], names: &[(char, &str)]) {
    let mut counts: Vec<(char, usize)> = Vec::new();
    for (glyph, name) in names {
        let n = grid
            .iter()
            .map(|r| r.iter().filter(|c| **c == *glyph).count())
            .sum();
        counts.push((*glyph, n));
        out.push(format!("{glyph} {name} — {n}"));
    }
    let total: usize = counts.iter().map(|(_, n)| n).sum();
    out.push(String::new());
    out.push(format!("{total} cells total"));
}

// ------------------------------------------------------------------ terrain

fn value_noise(rng: &mut Rng, w: usize, h: usize) -> Vec<Vec<f64>> {
    // two octaves of smooth value noise: coarse random grids, bilinear interp
    let mut octave = |scale: usize| -> Vec<Vec<f64>> {
        let gw = w / scale + 2;
        let gh = h / scale + 2;
        let coarse: Vec<Vec<f64>> = (0..gh)
            .map(|_| (0..gw).map(|_| rng.random()).collect())
            .collect();
        let mut g = vec![vec![0.0; w]; h];
        for (y, row) in g.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let fx = x as f64 / scale as f64;
                let fy = y as f64 / scale as f64;
                let x0 = fx as usize;
                let y0 = fy as usize;
                let tx = fx - x0 as f64;
                let ty = fy - y0 as f64;
                let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
                let top = lerp(coarse[y0][x0], coarse[y0][x0 + 1], tx);
                let bot = lerp(coarse[y0 + 1][x0], coarse[y0 + 1][x0 + 1], tx);
                *cell = lerp(top, bot, ty);
            }
        }
        g
    };
    let base = octave(6);
    let fine = octave(3);
    let mut g = vec![vec![0.0; w]; h];
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for y in 0..h {
        for x in 0..w {
            g[y][x] = 0.75 * base[y][x] + 0.25 * fine[y][x];
            lo = lo.min(g[y][x]);
            hi = hi.max(g[y][x]);
        }
    }
    // normalize to 0..1 so every biome band can actually appear
    let span = (hi - lo).max(1e-9);
    for row in &mut g {
        for c in row.iter_mut() {
            *c = (*c - lo) / span;
        }
    }
    g
}

fn terrain(spec: &Value, out: &mut Vec<String>) -> Result<(), String> {
    let (w, h) = dims(spec, 40, 16)?;
    let mut rng = Rng::from_spec(spec);
    let noise = value_noise(&mut rng, w, h);
    let names: Vec<String> = spec
        .get("biomes")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().map(crate::render::scalar_text).collect())
        .unwrap_or_else(|| {
            vec![
                "sea".into(),
                "beach".into(),
                "plains".into(),
                "forest".into(),
                "mountains".into(),
            ]
        });
    if names.len() != 5 {
        return Err(format!(
            "map.biomes must list exactly 5 names (one per band), got {}",
            names.len()
        ));
    }
    const GLYPHS: [char; 5] = ['≈', '░', '▒', '▓', '▲'];
    const CUTS: [f64; 4] = [0.30, 0.45, 0.70, 0.85];
    let mut grid = vec![vec![' '; w]; h];
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            let v = noise[y][x];
            let band = CUTS.iter().filter(|&&c| v < c).count();
            *cell = GLYPHS[band];
        }
    }
    push_grid(out, &grid);
    out.push(String::new());
    let legend: Vec<(char, &str)> = GLYPHS
        .iter()
        .copied()
        .zip(names.iter().map(|s| s.as_str()))
        .collect();
    push_legend(out, &grid, &legend);
    Ok(())
}

// ------------------------------------------------------------------ dungeon

fn dungeon(spec: &Value, out: &mut Vec<String>) -> Result<(), String> {
    let (w, h) = dims(spec, 48, 20)?;
    let target = spec
        .get("rooms")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize)
        .unwrap_or(6)
        .clamp(1, 50);
    let mut rng = Rng::from_spec(spec);
    let mut grid = vec![vec!['#'; w]; h];
    let mut rooms: Vec<(usize, usize, usize, usize)> = Vec::new(); // x, y, w, h
    for _ in 0..target {
        for _try in 0..24 {
            let rw = 4 + rng.below(5);
            let rh = 3 + rng.below(3);
            let rx = 1 + rng.below(w.saturating_sub(rw + 2));
            let ry = 1 + rng.below(h.saturating_sub(rh + 2));
            let overlap = rooms.iter().any(|&(x, y, rw2, rh2)| {
                rx < x + rw2 + 1 && x < rx + rw + 1 && ry < y + rh2 + 1 && y < ry + rh + 1
            });
            if !overlap {
                for row in &mut grid[ry..ry + rh] {
                    row[rx..rx + rw].fill('.');
                }
                rooms.push((rx, ry, rw, rh));
                break;
            }
        }
    }
    if rooms.is_empty() {
        return Err("dungeon failed to place any room (w/h too small?)".into());
    }
    // connect consecutive room centers with L corridors (rng picks the elbow)
    let center = |r: &(usize, usize, usize, usize)| (r.0 + r.2 / 2, r.1 + r.3 / 2);
    for pair in rooms.windows(2) {
        let (x0, y0) = center(&pair[0]);
        let (x1, y1) = center(&pair[1]);
        let horiz_first = rng.random() < 0.5;
        let carve = |x: usize, y: usize, g: &mut Vec<Vec<char>>| {
            g[y][x] = '.';
        };
        let step_x = |from: usize, to: usize, y: usize, g: &mut Vec<Vec<char>>| {
            for x in from.min(to)..=from.max(to) {
                carve(x, y, g);
            }
        };
        let step_y = |from: usize, to: usize, x: usize, g: &mut Vec<Vec<char>>| {
            for y in from.min(to)..=from.max(to) {
                carve(x, y, g);
            }
        };
        if horiz_first {
            step_x(x0, x1, y0, &mut grid);
            step_y(y0, y1, x1, &mut grid);
        } else {
            step_y(y0, y1, x0, &mut grid);
            step_x(x0, x1, y1, &mut grid);
        }
    }
    push_grid(out, &grid);
    out.push(String::new());
    let floor: usize = grid
        .iter()
        .map(|r| r.iter().filter(|c| **c == '.').count())
        .sum();
    out.push(format!(
        "# {} rooms, {} walkable cells of {} ({wall} walls)",
        rooms.len(),
        floor,
        w * h,
        wall = w * h - floor
    ));
    Ok(())
}

// --------------------------------------------------------------------- maze

fn maze(spec: &Value, out: &mut Vec<String>) -> Result<(), String> {
    // odd dimensions so cells and walls alternate; normalize with | 1
    let (w, h) = dims(spec, 41, 15)?;
    let (w, h) = (w | 1, h | 1);
    let mut rng = Rng::from_spec(spec);
    let mut grid = vec![vec!['█'; w]; h];
    // recursive backtracker over cell lattice (odd coordinates)
    let mut stack: Vec<(usize, usize)> = vec![(1, 1)];
    grid[1][1] = ' ';
    let dirs: [(isize, isize); 4] = [(2, 0), (-2, 0), (0, 2), (0, -2)];
    while let Some(&(x, y)) = stack.last() {
        let mut next: Vec<(isize, isize)> = dirs
            .iter()
            .filter(|&&(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                nx > 0
                    && ny > 0
                    && (nx as usize) < w - 1
                    && (ny as usize) < h - 1
                    && grid[ny as usize][nx as usize] == '█'
            })
            .copied()
            .collect();
        if next.is_empty() {
            stack.pop();
            continue;
        }
        // deterministic shuffle via rng
        let j = rng.below(next.len());
        next.swap(0, j);
        let (dx, dy) = next[0];
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;
        let mx = (x as isize + dx / 2) as usize;
        let my = (y as isize + dy / 2) as usize;
        grid[my][mx] = ' ';
        grid[ny][nx] = ' ';
        stack.push((nx, ny));
    }
    push_grid(out, &grid);
    out.push(String::new());
    let open: usize = grid
        .iter()
        .map(|r| r.iter().filter(|c| **c == ' ').count())
        .sum();
    out.push(format!(
        "█ wall — perfect maze: every cell reachable, single component, {open} open cells"
    ));
    Ok(())
}
