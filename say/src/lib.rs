mod constants;
use constants::names::*;

pub fn encode(n: u64) -> String {
    if n < 100 {
        return encode_small(n);
    }

    encode_large(n)
}

fn encode_small(n: u64) -> String {
    if n < 20 {
        return SMALL[n as usize].into();
    }

    let tens = TENS[(n / 10) as usize];
    if n % 10 == 0 {
        return tens.into();
    }

    format!("{}-{}", tens, encode_small(n % 10))
}

fn encode_large(n: u64) -> String {
    let mut result = String::new();
    let mut remaining = n;
    let mut scale_idx = 0;

    while remaining > 0 {
        // Chunks of 3
        let chunk = remaining % 1000;

        if chunk > 0 {
            result = format!("{} {}", format_chunk(chunk, scale_idx), result);
        }

        remaining /= 1000;
        scale_idx += 1;
    }

    result.trim().into()
}

fn format_chunk(chunk: u64, scale_idx: usize) -> String {
    let mut result = String::new();

    // Chunk out the hundreds part
    let hundreds = chunk / 100;
    let remainder = chunk % 100;

    // Add the hundreds part
    if hundreds > 0 {
        result = format!("{} hundred", encode_small(hundreds));
    }

    // Add the tens and ones
    if remainder > 0 {
        if hundreds > 0 {
            result = format!("{} {}", result, encode_small(remainder));
        } else {
            result = encode_small(remainder);
        }
    }

    // Insert the appropriate scale in between results
    if scale_idx > 0 {
        result = format!("{} {}", result, SCALES[scale_idx])
    }

    result
}
