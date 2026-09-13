use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Span {
    value: Option<i32>,
    start: usize,
    len: usize,
}

fn main() -> Result<(), String> {
    let file = File::open("input.txt").map_err(|err| format!("failed to open input.txt: {err}"))?;
    let reader = BufReader::new(file);
    let line = reader
        .lines()
        .flatten()
        .next()
        .ok_or("unable to read first line")?;

    let mut block_id = 0;
    let mut filled_blocks: Vec<Span> = vec![];
    let mut empty_blocks: Vec<Span> = vec![];

    let mut pos: usize = 0;

    for (idx, ch) in line.chars().enumerate() {
        let len = ch
            .to_digit(10)
            .ok_or_else(|| format!("invalid digit '{ch}' at postition '{idx}'"))?
            as usize;

        if idx % 2 == 0 {
            filled_blocks.push(Span {
                value: Some(block_id),
                start: pos,
                len,
            });
            block_id += 1;
        } else if len > 0 {
            empty_blocks.push(Span {
                value: None,
                start: pos,
                len,
            });
        }

        pos += len;
    }

    let mut collected: Vec<Span> = vec![];
    while let Some(mut block) = filled_blocks.pop() {
        let possible_empty_block = empty_blocks
            .iter_mut()
            .find(|empty_block| empty_block.start < block.start && empty_block.len >= block.len);

        if let Some(empty_block) = possible_empty_block {
            let empty_block_start = empty_block.start;
            empty_block.start += block.len;
            empty_block.len -= block.len;

            empty_blocks.push(Span {
                value: None,
                start: block.start,
                len: block.len,
            });

            block.start = empty_block_start;
        }

        collected.push(block);
    }

    let result = collected.into_iter().fold(0 as i64, |acc, block| {
        acc + block.value.map_or(0i64, |val| {
            val as i64 * block.len as i64 * (2 * block.start as i64 + block.len as i64 - 1) / 2
        })
    });

    dbg!(result);

    Ok(())
}
