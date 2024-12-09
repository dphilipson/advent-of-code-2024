use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    let mut digits = input.single_line(|line| line.digits());
    digits.push(0);
    let mut ids = Vec::with_capacity(digits.iter().sum());
    for (id, window) in digits.chunks(2).enumerate() {
        let (on, off) = (window[0], window[1]);
        for _ in 0..on {
            ids.push(Some(id));
        }
        for _ in 0..off {
            ids.push(None);
        }
    }
    let mut left = 0;
    let mut right = ids.len() - 1;
    while left < right {
        if ids[left].is_some() {
            left += 1;
            continue;
        }
        if ids[right].is_none() {
            right -= 1;
            continue;
        }
        ids.swap(left, right);
    }
    ids.into_iter()
        .enumerate()
        .map(|(i, id)| i * id.unwrap_or_default())
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let mut digits = input.single_line(|line| line.digits());
    digits.push(0);

    #[derive(Copy, Clone, Debug)]
    struct Span {
        id: usize,
        location: usize,
        length: usize,
    }

    #[derive(Copy, Clone, Debug)]
    struct Gap {
        location: usize,
        length: usize,
    }

    let mut spans = Vec::new();
    let mut gaps = Vec::new();
    let mut location = 0;
    for (id, window) in digits.chunks(2).enumerate() {
        let (span, gap) = (window[0], window[1]);
        spans.push(Span {
            id,
            location,
            length: span,
        });
        gaps.push(Gap {
            location: location + span,
            length: gap,
        });
        location += span + gap;
    }
    for span in spans.iter_mut().rev() {
        let Some(first_gap) = gaps.iter_mut().find(|gap| gap.length >= span.length) else {
            continue;
        };
        if first_gap.location < span.location {
            span.location = first_gap.location;
            first_gap.location += span.length;
            first_gap.length -= span.length;
        }
    }
    spans
        .into_iter()
        .map(|span| span.id * (span.location..span.location + span.length).sum::<usize>())
        .sum()
}
