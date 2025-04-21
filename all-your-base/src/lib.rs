#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if number.is_empty() {
        return Ok(vec![0]);
    }

    let decimal_value = convert_to_decimal(number, from_base)?;
    let output = convert_to_base(decimal_value, to_base);

    Ok(output)
}

fn convert_to_decimal(number: &[u32], from_base: u32) -> Result<u32, Error> {
    number
        .iter()
        .rev()
        .enumerate()
        .try_fold(0, |acc, (idx, &digit)| {
            if digit >= from_base {
                return Err(Error::InvalidDigit(digit));
            }
            Ok(acc + digit * from_base.pow(idx as u32))
        })
}

fn convert_to_base(mut decimal_value: u32, to_base: u32) -> Vec<u32> {
    if decimal_value == 0 {
        return vec![0];
    }

    let mut remainders = Vec::new();
    while decimal_value > 0 {
        remainders.push(decimal_value % to_base);
        decimal_value /= to_base;
    }
    remainders.reverse();
    remainders
}
