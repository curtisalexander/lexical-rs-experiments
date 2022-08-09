use lexical::write_float_options::RoundMode;
use std::error::Error;
use std::num::NonZeroI32;
// use std::num::NonZeroUsize;

fn print_numerics(f: f64) {
    println!("\nf (initially) = {}", f);

    const FORMAT: u128 = lexical::format::STANDARD;
    // const DIGITS: Option<NonZeroUsize> = std::num::NonZeroUsize::new(14 as usize);
    const EXPBREAK: Option<NonZeroI32> = std::num::NonZeroI32::new(14i32);

    let options = lexical::WriteFloatOptions::builder()
        .trim_floats(true)
        // .max_significant_digits(DIGITS)
        .positive_exponent_break(EXPBREAK)
        .round_mode(RoundMode::Truncate)
        .build()
        .unwrap();
    let f_as_str_with_options = lexical::to_string_with_options::<_, FORMAT>(f, &options);
    println!("f (after converting to string) = {}", f_as_str_with_options);

    let f_as_f: f64 = lexical::parse(f_as_str_with_options).unwrap();
    println!("f (after converting back to float) = {}", f_as_f);
}

fn print_numerics_prior(f: f64) -> Result<(), Box<dyn Error>> {
    println!("\nf (initially) = {}", f);
    const DIGITS: usize = 14;
    let f_as_str = format!(
        "{1:.0$}",
        DIGITS,
        f //f64::trunc(f * 10f64.powi(DIGITS as i32)) / 10f64.powi(DIGITS as i32)
    );
    println!("f (after converting to string with format macro) = {}", f);

    let f_as_f: f64 = lexical::parse(f_as_str)?;
    println!("f (after converting back to float) = {}", f_as_f);

    Ok(())
}

fn print_numerics_another(f: f64) -> Result<(), Box<dyn Error>> {
    println!("\nf (initially) = {}", f);
    const DIGITS: usize = 14;
    let f_as_str = format!(
        "{1:.0$}",
        DIGITS,
        f //f64::trunc(f * 10f64.powi(DIGITS as i32)) / 10f64.powi(DIGITS as i32)
    );
    println!("f (after converting to string with format macro) = {}", f);

    let f_as_f: f64 = f_as_str.parse::<f64>()?;
    println!("f (after converting back to float) = {}", f_as_f);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = 333039375527f64;
    print_numerics(f);

    let f = 99.1234567890123456;
    print_numerics(f);

    let f = 333039375527f64;
    print_numerics_prior(f)?;

    let f = 99.1234567890123456;
    print_numerics_prior(f);

    let f = 4.6000000000000087;
    print_numerics_prior(f);

    let f = 4.6000000000000087;
    print_numerics(f);

    let f = 4.6000000000000087;
    print_numerics_another(f);

    Ok(())
}
