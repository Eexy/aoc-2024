use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), String> {
    let file = File::open("input.txt").map_err(|err| format!("failed to open input.txt: {err}"))?;
    let reader = BufReader::new(file);
    let line = reader
        .lines()
        .flatten()
        .next()
        .ok_or("unable to read first line")?;
    let chars = line.chars();

    let mut block_id = 0;
    let mut filled_block: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut empty_blocks: Vec<(i32, i32)> = vec![];

    let mut pos = 0;

    let mut collected = chars
        .into_iter()
        .map(|ch| ch as i32 - ('0' as i32))
        .enumerate()
        .flat_map(|(idx, ch)| {
            let iter = if idx % 2 == 0 {
                std::iter::repeat_n(Some(block_id), ch as usize)
            } else {
                std::iter::repeat_n(None, ch as usize)
            };

            if idx % 2 == 0 {
                filled_block.insert(block_id, (pos, ch as i32));
                block_id += 1;
            } else {
                empty_blocks.push((pos, ch as i32));
            }

            pos += ch;
            iter
        })
        .collect::<Vec<_>>();

    while block_id != 0 {
        match filled_block.get(&block_id) {
            Some(block) => {
                let position_empty_blocks = empty_blocks
                    .iter_mut()
                    .find(|empty_block| empty_block.0 < block.0 && empty_block.1 >= block.1);

                if let Some(empty_block) = position_empty_blocks {
                    // swap slice
                    collected.splice(
                        empty_block.0 as usize..(empty_block.0 + block.1) as usize,
                        std::iter::repeat_n(Some(block_id), block.1 as usize),
                    );

                    collected.splice(
                        block.0 as usize..(block.0 + block.1) as usize,
                        std::iter::repeat_n(None, block.1 as usize),
                    );

                    // fill empty block
                    empty_block.0 += block.1;
                    empty_block.1 -= block.1;

                    // create new empty block where filled block where
                    empty_blocks.push((block.0, block.1));

                    // fill vector
                }
            }
            None => {}
        }
        block_id -= 1;
    }

    let result = collected
        .into_iter()
        .flat_map(|val| val.or(Some(0)))
        .enumerate()
        .fold(0 as i64, |acc, (idx, val)| acc + (idx as i32 * val) as i64);

    dbg!(result);

    Ok(())
}
