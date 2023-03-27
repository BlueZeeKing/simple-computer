use simple_computer::{ SimpleComputer, commands::Commands };

#[test]
fn full_test_1() {
    let mut out = Vec::new();

    SimpleComputer::new(
        Commands::parse_commands("plus\nplus\nplus\nminus\nminus\nminus\nback 6\noutput\nback 1"),
        &mut out
    ).for_each(|value| value.unwrap());

    assert_eq!(String::from_utf8_lossy(&out).trim(), "0\n0");
}

#[test]
fn full_test_2() {
    let mut out = Vec::new();

    SimpleComputer::new(
        Commands::parse_commands("plus\nback 1\nback 2\noutput"),
        &mut out
    ).for_each(|value| value.unwrap());

    assert_eq!(String::from_utf8_lossy(&out).trim(), "3")
}