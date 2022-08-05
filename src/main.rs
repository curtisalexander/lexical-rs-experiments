use lexical;

fn main() {
    let f_as_str = lexical::to_string(3.0);
    println!("f_as_str = {}", &f_as_str);

    let str_as_f: f64 = lexical::parse("3.5").unwrap();
    println!("str_as_f = {}", str_as_f);

    let str_as_f_sn: f64 = lexical::parse("333039375527").unwrap();
    println!("str_as_f_sn = {}", str_as_f_sn);

    const FORMAT: u128 = lexical::format::STANDARD;
    let options = lexical::WriteFloatOptions::builder()
        .trim_floats(true)
        .build()
        .unwrap();
    let str_with_options = lexical::to_string_with_options::<_, FORMAT>(0.0, &options);
    println!("str_with_options = {}", str_with_options);

    let str_with_options2 = lexical::to_string_with_options::<_, FORMAT>(123.456, &options);
    println!("str_with_options2 = {}", str_with_options2);

    let sn_value = 333039375527f64;
    println!("sn_value = {}", sn_value);

    // errors when DIGITS = 14
    const DIGITS: usize = 14;
    let sn_as_str = format!("{1:.0$}", DIGITS, lexical::to_string(sn_value));
    println!("sn_as_str = {}", sn_as_str);

    let sn_as_f: f64 = lexical::parse(sn_as_str).unwrap();
    println!("sn_as_f = {}", sn_as_f);
}
