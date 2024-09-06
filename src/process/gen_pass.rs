use rand::{seq::SliceRandom, thread_rng};

const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const SYMBOL: &[u8] = b"!@#$%^&*_";
const NUMBER: &[u8] = b"123456789";

pub fn process_genpass(
    length: u8,
    noupper: bool,
    nolower: bool,
    nonumber: bool,
    nosymbol: bool,
) -> anyhow::Result<String> {
    let mut password = Vec::new();
    let mut rng = thread_rng();
    let mut chars = Vec::<u8>::new();

    if !nolower {
        chars.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("LOWER won't be empty in this context"),
        );
    }
    if !noupper {
        chars.extend_from_slice(UPPER);
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    if !nonumber {
        chars.extend_from_slice(NUMBER);
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("NUMBER won't be empty in this context"),
        );
    }
    if !nosymbol {
        chars.extend_from_slice(SYMBOL);
        password.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("SYMBOL won't be empty in this context"),
        );
    }

    for _ in 0..(length - password.len() as u8) {
        password.push(
            *chars
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    password.shuffle(&mut rng);

    let password: String = String::from_utf8(password)?;

    Ok(password)
}
