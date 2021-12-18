#![allow(dead_code)]
use std::env;
use std::fs;
use std::io;

use std::cmp::{max, min};

use std::io::BufRead;

use anyhow::Result;

pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

fn fn1a() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let mut lines = io::BufReader::new(f).lines();

    let mut inc = 0;
    let mut prev: i64 = lines.next().unwrap()?.parse()?;
    for l in lines {
        // just reuse if we get an unparsable response.
        let ln: i64 = l?.parse().unwrap_or(prev);
        if ln > prev {
            inc += 1;
        }
        prev = ln;
    }

    return Ok(format!("{}", inc));
}

fn fn1b() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = io::BufReader::new(f)
        .lines()
        .filter_map(|l| l.ok()?.parse().ok())
        .collect::<Vec<i64>>();

    let mut windows = lines.windows(3);

    let mut inc = 0;
    let mut prev: Vec<i64> = windows.next().unwrap().to_vec();
    for w in windows {
        let (p, c): (i64, i64) = (prev.iter().sum(), w.iter().sum());
        if c > p {
            inc += 1;
        }
        prev = w.to_vec();
    }

    return Ok(format!("{}", inc));
}

fn fn2a() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = io::BufReader::new(f).lines().filter_map(|l| {
        Some(
            l.ok()?
                .split(" ")
                .map(|st| st.to_string())
                .collect::<Vec<String>>(),
        )
    });

    let mut pos = (0, 0);

    for v in lines {
        match v[0].as_str() {
            "forward" => pos.0 += v[1].parse::<i64>()?,
            "down" => pos.1 -= v[1].parse::<i64>()?,
            "up" => pos.1 += v[1].parse::<i64>()?,
            _ => panic!(),
        };
    }

    return Ok(format!("{}", pos.0 * pos.1 * -1));
}

fn fn2b() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    // I hate it!
    let lines = io::BufReader::new(f).lines().filter_map(|l| {
        Some(
            l.ok()?
                .split(" ")
                .map(|st| st.to_string())
                .collect::<Vec<String>>(),
        )
    });

    let mut pos = (0, 0);
    let mut aim = 0;

    for v in lines {
        match v[0].as_str() {
            "forward" => {
                let x = v[1].parse::<i64>()?;
                pos.0 += x;
                pos.1 += aim * x;
            }
            "down" => aim += v[1].parse::<i64>()?,
            "up" => aim -= v[1].parse::<i64>()?,
            _ => panic!(),
        };
    }

    return Ok(format!("{}", pos.0 * pos.1));
}

fn fn3a() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = io::BufReader::new(f).lines();

    let mut count = 0;
    let mut bits = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for l in lines {
        count += 1;
        let ll = l?;
        for (i, a) in ll.chars().rev().enumerate() {
            bits[i] += match a {
                '1' => 1,
                _ => 0,
            };
        }
    }

    println!("bits: {:?}, count: {}", bits, count);

    let mut gamma = 0;
    for i in 0..bits.len() {
        gamma += i64::pow(2, i as u32) * if bits[i] > (count / 2) { 1 } else { 0 };
    }

    let epsi = (!gamma) & 0b1111_1111_1111;

    println!("e:{} + {:b}", epsi, epsi);
    println!("g:{} + {:b}", gamma, gamma);
    return Ok(format!("{}", epsi * gamma));
}

fn fn3b() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let mut co2: Vec<String> = io::BufReader::new(f)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let mut o2 = co2.clone();

    let max_idx = o2[0].as_bytes().len();
    let mut idx = 0;
    let mut co2_best = "ERR".to_owned();
    while co2.len() > 0 && idx < max_idx {
        co2_best = co2[0].clone();
        let mut ones = 0;
        for l in &co2 {
            ones += if l.as_bytes()[idx] == '1' as u8 { 1 } else { 0 };
        }
        let common = if ones >= co2.len() - ones { '1' } else { '0' } as u8;
        co2.retain(|l| l.as_bytes()[idx] != common);
        idx += 1;
    }
    if co2.len() > 0 {
        co2_best = co2[0].clone();
    }

    idx = 0;
    let mut o2_best = "ERR".to_owned();
    while o2.len() > 0 && idx < max_idx {
        o2_best = o2[0].clone();
        let mut ones = 0;
        for l in &o2 {
            ones += if l.as_bytes()[idx] == '1' as u8 { 1 } else { 0 };
        }
        let common = if ones >= o2.len() - ones { '1' } else { '0' } as u8;
        o2.retain(|l| l.as_bytes()[idx] == common);
        idx += 1;
    }
    if o2.len() > 0 {
        o2_best = o2[0].clone();
    }

    return Ok(format!(
        "{} {}: {}",
        o2_best,
        co2_best,
        i64::from_str_radix(&co2_best, 2)? * i64::from_str_radix(&o2_best, 2)?
    ));
}

fn fn4a() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let sent = 100; // no board has 100.

    let calls = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|e| e.parse().unwrap())
        .collect::<Vec<i32>>();

    let boards = &mut Vec::<Vec<i32>>::new();
    while let Some(_) = lines.next() {
        let mut b = Vec::<i32>::new();
        lines.take(5).for_each(|e| {
            b.extend(
                e.unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap()),
            )
        });
        boards.push(b);
    }

    for call in &calls {
        for b in &mut *boards {
            for e in b.iter_mut() {
                if *e == *call {
                    *e = sent;
                }
            }
            // hori
            for e in b.chunks(5) {
                if e.iter().fold(true, |s, x| s && *x == 100) {
                    let unused = b.iter().fold(0, |s, &e| if e != 100 { s + e } else { s });
                    return Ok(format!("{}", call * unused));
                }
            }
            // vert
            for i in 0..5 {
                let mut solved = true;
                for y in 0..5 {
                    solved &= b[i + 5 * y] == 100;
                }
                if solved {
                    let unused = b.iter().fold(0, |s, &e| if e != 100 { s + e } else { s });
                    return Ok(format!("{}", call * unused));
                }
            }
        }
    }

    return Ok(format!("{:?}", boards));
}

fn fn4b() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let sent = 100; // no board has 100.

    let calls = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|e| e.parse().unwrap())
        .collect::<Vec<i32>>();

    let boards = &mut Vec::<(bool, Vec<i32>)>::new();
    while let Some(_) = lines.next() {
        let mut b = Vec::<i32>::new();
        lines.take(5).for_each(|e| {
            b.extend(
                e.unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap()),
            )
        });
        boards.push((false, b));
    }

    let mut bcount = boards.len();

    for call in &calls {
        for (s, b) in &mut *boards {
            if *s {
                continue;
            }
            for e in b.iter_mut() {
                if *e == *call {
                    *e = sent;
                }
            }
            // hori
            for e in b.chunks(5) {
                if e.iter().fold(true, |s, x| s && *x == sent) {
                    *s = true
                }
            }
            // vert
            for i in 0..5 {
                let mut solved = true;
                for y in 0..5 {
                    solved &= b[i + 5 * y] == sent;
                }
                if solved {
                    *s = solved
                }
            }

            if *s {
                bcount -= 1;
                if bcount == 0 {
                    let unused = b.iter().fold(0, |s, &e| if e != sent { s + e } else { s });
                    println!("{}, {}, {:?}", call, unused, b);
                    return Ok(format!("{}", call * unused));
                }
            }
        }
    }

    return Ok(format!("{:?}", boards));
}

fn fn5a() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let mut vents: Vec<((i32, i32), (i32, i32))> = lines
        .map(|l| {
            let i = l.unwrap();
            let mut pi = i.split(" -> ").map(|p| {
                let mut ps = p.split(",").map(|pp| pp.parse::<i32>().unwrap());
                (ps.next().unwrap(), ps.next().unwrap())
            });
            (pi.next().unwrap(), pi.next().unwrap())
        })
        .collect();

    // Only consider hori/vert
    vents.retain(|e| e.0 .0 == e.1 .0 || e.0 .1 == e.1 .1);

    let xmax = vents.iter().map(|e| max(e.0 .0, e.1 .0)).max().unwrap() + 1;
    let ymax = vents.iter().map(|e| max(e.0 .1, e.1 .1)).max().unwrap() + 1;

    let mut field = vec![0u32; (xmax * ymax) as usize];
    // trace lines.
    for v in &vents {
        let (mut x, mut y) = (min(v.0 .0, v.1 .0), min(v.0 .1, v.1 .1));
        let (x2, y2) = (max(v.0 .0, v.1 .0), max(v.0 .1, v.1 .1));
        let (dx, dy) = ((v.0 .0 - v.1 .0).abs(), (v.1 .1 - v.0 .1).abs());
        let len = ((dx.pow(2) + dy.pow(2)) as f64).sqrt() as i32; // meh.
        let (tx, ty) = (dx / len, dy / len);

        while x <= x2 && y <= y2 {
            field[(x + ymax * y) as usize] += 1;
            x += tx;
            y += ty;
        }
    }

    field.retain(|&e| e >= 2);

    return Ok(format!("{:?}", field.len()));
}

fn fn5b() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let vents: Vec<((i32, i32), (i32, i32))> = lines
        .map(|l| {
            let i = l.unwrap();
            let mut pi = i.split(" -> ").map(|p| {
                let mut ps = p.split(",").map(|pp| pp.parse::<i32>().unwrap());
                (ps.next().unwrap(), ps.next().unwrap())
            });
            (pi.next().unwrap(), pi.next().unwrap())
        })
        .collect();

    let xmax = vents.iter().map(|e| max(e.0 .0, e.1 .0)).max().unwrap() + 1;
    let ymax = vents.iter().map(|e| max(e.0 .1, e.1 .1)).max().unwrap() + 1;

    let mut field = vec![0u32; (xmax * ymax) as usize];
    // trace lines.
    for v in &vents {
        let (mut x, mut y) = (v.0 .0, v.0 .1);
        let (x2, y2) = (v.1 .0, v.1 .1);
        let (dx, dy) = (x2 - x, y2 - y);
        let (tx, ty) = (clamp(dx, -1, 1), clamp(dy, -1, 1));

        while x != x2 || y != y2 {
            field[(x + ymax * y) as usize] += 1;
            x += tx;
            y += ty;
        }
        field[(x + ymax * y) as usize] += 1;
    }

    /*
    println!(
        "{}",
        field
            .chunks(xmax as usize)
            .map(|e| e
                .iter()
                .map(|ee| format!("{}", ee))
                .collect::<Vec<String>>()
                .join(","))
            .collect::<Vec<String>>()
            .join("\n")
    );
    */
    field.retain(|&e| e >= 2);

    return Ok(format!("{:?}", field.len()));
}

fn fn6ab() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let mut fish = vec![0; 10];
    lines
        .next()
        .unwrap()?
        .split(",")
        .map(|e| e.parse::<i64>().unwrap())
        .for_each(|e| fish[(e + 1) as usize] += 1);

    // 80 for a, 256 for b
    for _ in 0..256 {
        for f in 0..9 {
            fish[f] = fish[f + 1];
        }
        let spawn = fish[0];
        fish[0] = 0;
        fish[7] += spawn;
        fish[9] = spawn;
    }

    return Ok(format!("{:?}", fish.iter().sum::<i64>()));
}

fn fn7a() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let input = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let max = *input.iter().max().unwrap();
    let mut cost = vec![0; max as usize];
    for i in &input {
        for x in 0..max {
            cost[x as usize] += (x - i).abs();
        }
    }

    // println!("{:?}", cost);

    let best = cost
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(i, _)| i as i32)
        .unwrap();

    let best_cost = input.iter().map(|&e| (best - e).abs()).sum::<i32>();

    return Ok(format!("{},{}", best, best_cost));
}

fn fn7b() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let input = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let max = *input.iter().max().unwrap();
    let mut cost = vec![0; max as usize];
    for i in &input {
        for x in 0..max {
            let n = (x - i).abs();
            cost[x as usize] += (n * (n + 1)) / 2;
        }
    }

    let best = cost
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(i, _)| i as i32)
        .unwrap();

    let best_cost = input
        .iter()
        .map(|&e| {
            let n = (best - e).abs();
            return (n * (n + 1)) / 2;
        })
        .sum::<i32>();

    return Ok(format!("{},{}", best, best_cost));
}

fn char_matches(a: &str, b: &str) -> u32 {
    let mut matches = 0;
    for ac in a.chars() {
        for bc in b.chars() {
            if ac == bc {
                matches += 1;
            }
        }
    }
    return matches;
}

fn fn8ab() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let mut total = 0;
    for line in lines {
        let l = line?;
        let pats = l
            .split(" ")
            .take_while(|&e| e != "|")
            .collect::<Vec<&str>>();
        let mut nums: Vec<Option<&str>> = vec![None; 10];
        for p in &pats {
            if p.len() == 2 {
                nums[1] = Some(p);
            }
            if p.len() == 3 {
                nums[7] = Some(p);
            }
            if p.len() == 4 {
                nums[4] = Some(p);
            }
            if p.len() == 7 {
                nums[8] = Some(p);
            }
        }
        for p in &pats {
            if p.len() == 5 {
                // 5: 2,3,5
                if char_matches(p, nums[1].unwrap()) == 2 {
                    nums[3] = Some(p);
                    continue;
                }
                if char_matches(p, nums[4].unwrap()) == 2 {
                    nums[2] = Some(p);
                    continue;
                }
                if char_matches(p, nums[4].unwrap()) == 3 {
                    nums[5] = Some(p);
                    continue;
                }
            }
            if p.len() == 6 {
                // 6: 0,6,9
                if char_matches(p, nums[1].unwrap()) == 1 {
                    nums[6] = Some(p);
                    continue;
                }
                if char_matches(p, nums[4].unwrap()) == 4 {
                    nums[9] = Some(p);
                    continue;
                }
                if char_matches(p, nums[1].unwrap()) == 2 {
                    nums[0] = Some(p);
                    continue;
                }
            }
        }

        // println!("{:?}", nums);
        let outs = l
            .split(" ")
            .skip_while(|&e| e != "|")
            .skip(1)
            .map(|e| {
                let mut o = 0;
                for (i, &n) in nums.iter().enumerate() {
                    if char_matches(e, n.unwrap()) as usize == e.len()
                        && n.unwrap().len() == e.len()
                    {
                        o = i;
                    }
                }
                return o;
            })
            .collect::<Vec<usize>>();
        total += outs[0] * 1000 + outs[1] * 100 + outs[2] * 10 + outs[3];
        // println!("{:?}", outs);
    }

    return Ok(format!("{}", total));
}

fn fn9a() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let bounds_val = 255;
    let mut map_data = lines
        .map(|e| {
            let mut r = e
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>();
            r.insert(0, bounds_val);
            r.push(bounds_val);
            return r;
        })
        .collect::<Vec<Vec<u8>>>();
    let xmax = map_data[0].len() - 2;
    let ymax = map_data.len();
    map_data.insert(0, vec![bounds_val; xmax + 2]);
    map_data.push(vec![bounds_val; xmax + 2]);

    println!("{},{}", xmax, ymax);

    let mut mins = Vec::new();
    let dirs: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
    for x in 1..xmax + 1 {
        for y in 1..ymax + 1 {
            let mut is_min = true;
            for &(dx, dy) in dirs {
                if map_data[y][x] >= map_data[(dy + y as i32) as usize][(dx + x as i32) as usize] {
                    is_min = false;
                    break;
                }
            }
            if is_min {
                mins.push((x, y));
            }
        }
    }

    println!("{:?}", mins);

    let mut risk = 0u32;
    for (x, y) in mins {
        risk += map_data[y][x] as u32 + 1;
    }

    return Ok(format!("{}", risk));
}

fn fn9b() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let bounds_val = 999;
    let mut map_data = lines
        .map(|e| {
            let mut r = e
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i16)
                .collect::<Vec<i16>>();
            r.insert(0, bounds_val);
            r.push(bounds_val);
            return r;
        })
        .collect::<Vec<Vec<i16>>>();
    let xmax = map_data[0].len() - 2;
    let ymax = map_data.len();
    map_data.insert(0, vec![bounds_val; xmax + 2]);
    map_data.push(vec![bounds_val; xmax + 2]);

    println!("{},{}", xmax, ymax);

    let mut mins = Vec::new();
    let dirs: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
    for x in 1..xmax + 1 {
        for y in 1..ymax + 1 {
            let mut is_min = true;
            for &(dx, dy) in dirs {
                if map_data[y][x] >= map_data[(dy + y as i32) as usize][(dx + x as i32) as usize] {
                    is_min = false;
                    break;
                }
            }
            if is_min {
                mins.push((x as i32, y as i32));
            }
        }
    }

    // println!("{:?}", mins);

    let mut basin_sizes = Vec::new();
    let mut basin = 0;
    for m in mins {
        let mut size = 0;
        let mut edges = vec![m; 1];
        while edges.len() > 0 {
            let e = edges.pop().unwrap();
            for &(dx, dy) in dirs {
                let (xn, yn) = (e.0 + dx, e.1 + dy);
                if map_data[yn as usize][xn as usize] < 9 {
                    map_data[yn as usize][xn as usize] = 10 + basin;
                    edges.push((xn, yn));
                    size += 1;
                }
            }
        }
        basin += 1;
        basin_sizes.push(size);
    }

    /*
    println!(
        "{}",
        map_data
            .iter()
            .map(|e| e
                .iter()
                .map(|ee| format!("{:3} ", ee))
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
    */

    basin_sizes.sort();
    return Ok(format!(
        "{:?},{:?}",
        basin_sizes,
        basin_sizes.iter().rev().take(3).fold(1, |a, e| a * e)
    ));
}

fn fn10a() -> Result<String> {
    use std::collections::HashMap;

    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let mut pairs = HashMap::new();
    pairs.insert(')', '(');
    pairs.insert(']', '[');
    pairs.insert('}', '{');
    pairs.insert('>', '<');
    let mut vals = HashMap::new();
    vals.insert(')', 3);
    vals.insert(']', 57);
    vals.insert('}', 1197);
    vals.insert('>', 25137);
    let mut corrupt_score = 0;
    for l in lines {
        let mut stack = Vec::new();
        let mut corrupt = false;
        for c in l.unwrap().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let e = stack.pop().unwrap();
                    if pairs[&c] != e {
                        println!("unexpected {}", c);
                        corrupt_score += vals[&c];
                        corrupt = true;
                    }
                }
                _ => panic!(),
            }
            if corrupt {
                break;
            }
        }
    }

    return Ok(format!("{}", corrupt_score));
}

fn fn10b() -> Result<String> {
    use std::collections::HashMap;

    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    let mut pairs = HashMap::new();
    pairs.insert(')', '(');
    pairs.insert(']', '[');
    pairs.insert('}', '{');
    pairs.insert('>', '<');
    let mut vals = HashMap::new();
    vals.insert('(', 1);
    vals.insert('[', 2);
    vals.insert('{', 3);
    vals.insert('<', 4);
    let mut fix_scores = Vec::new();
    for l in lines {
        let mut stack = Vec::new();
        let mut corrupt = false;
        for c in l.unwrap().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let e = stack.pop().unwrap();
                    if pairs[&c] != e {
                        corrupt = true;
                    }
                }
                _ => panic!(),
            }
            if corrupt {
                break;
            }
        }
        if !corrupt && stack.len() > 0 {
            fix_scores.push(stack.iter().rev().fold(0u64, |acc, e| acc * 5 + vals[e]));
        }
    }

    fix_scores.sort();
    return Ok(format!(
        "{:?}",
        fix_scores.iter().nth(fix_scores.len() / 2).unwrap()
    ));
}

fn fn11ab() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();
    let dirs: &[(i32, i32)] = &[
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        // diag
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut octo_map = lines
        .map(|e| {
            e.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i16)
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();

    let xmax = octo_map[0].len();
    let ymax = octo_map.len();

    let mut sync = 0;
    let mut step = 0;
    while sync == 0 {
        step += 1;
        for x in 0..xmax {
            for y in 0..ymax {
                octo_map[y][x] += 1;
            }
        }
        let mut done = false;
        while !done {
            done = true;
            for x in 0..xmax {
                for y in 0..ymax {
                    if octo_map[y][x] > 9 {
                        done = false;
                        octo_map[y][x] = -1;
                        for (dx, dy) in dirs {
                            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                            if nx >= 0
                                && nx < xmax as i32
                                && ny >= 0
                                && ny < ymax as i32
                                && octo_map[ny as usize][nx as usize] != -1
                            {
                                octo_map[ny as usize][nx as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
        let mut flashed = 0;
        for x in 0..xmax {
            for y in 0..ymax {
                if octo_map[y][x] == -1 {
                    flashed += 1;
                    octo_map[y][x] = 0;
                }
            }
        }
        if flashed == xmax * ymax {
            sync = step;
        }
    }
    /*
    println!(
        "{}",
        octo_map
            .iter()
            .map(|e| e
                .iter()
                .map(|ee| format!("{} ", ee))
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
    */

    return Ok(format!("{:?}", sync));
}

fn fn12a() -> Result<String> {
    let ff = env::args().last().unwrap();
    let f = fs::File::open(&ff)?;
    let lines = &mut io::BufReader::new(f).lines();

    return Ok(format!("{:?}", sync));
}

fn main() -> Result<()> {
    println!("{}", fn12a()?);

    Ok(())
}
