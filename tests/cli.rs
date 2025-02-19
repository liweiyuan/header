use std::{
    error::Error,
    fs::{self, File},
    io::Read,
};

use assert_cmd::Command;
use predicates::prelude::predicate;
use rand::{distributions::Alphanumeric, Rng};

type HeaderResult<T> = Result<T, Box<dyn Error>>;

const PRG: &str = "header";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";
const TWO: &str = "./tests/inputs/two.txt";
const THREE: &str = "./tests/inputs/three.txt";
const TEN: &str = "./tests/inputs/ten.txt";

// --------------------------------------------------
fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn test_bad_bytes() -> HeaderResult<()> {
    let bad = gen_bad_file();
    let expected = format!("Failed to parse bytes count: {}", &bad);
    Command::cargo_bin(PRG)?
        .arg("-c")
        .arg(bad)
        .arg(EMPTY)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_lines() -> HeaderResult<()> {
    let bad = random_string();
    let expected = format!("Failed to parse lines count: {}", &bad);
    Command::cargo_bin(PRG)?
        .args(&["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_and_lines() -> HeaderResult<()> {
    let msg = "The argument '--lines <LINES>' cannot be \
               used with '--bytes <BYTES>'";

    Command::cargo_bin(PRG)?
        .args(&["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> HeaderResult<()> {
    let bad = gen_bad_file();
    let expected = format!("Failed to open file: {}", &bad);
    Command::cargo_bin(PRG)?
        .args([EMPTY, &bad, ONE])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> HeaderResult<()> {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));

    Ok(())
}

// --------------------------------------------------
fn run_stdin(args: &[&str], input_file: &str, expected_file: &str) -> HeaderResult<()> {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);
    let input = fs::read_to_string(input_file)?;

    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .assert()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> HeaderResult<()> {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n2() -> HeaderResult<()> {
    run(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")
}

// --------------------------------------------------
#[test]
fn empty_n4() -> HeaderResult<()> {
    run(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

// --------------------------------------------------
/* #[test]
fn empty_c2() -> HeaderResult<()> {
    run(&[EMPTY, "-c", "2"], "tests/expected/empty.txt.c2.out")
} */

// --------------------------------------------------
#[test]
fn empty_c4() -> HeaderResult<()> {
    run(&[EMPTY, "-c", "4"], "tests/expected/empty.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn one() -> HeaderResult<()> {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n2() -> HeaderResult<()> {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() -> HeaderResult<()> {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1() -> HeaderResult<()> {
    run(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2() -> HeaderResult<()> {
    run(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4() -> HeaderResult<()> {
    run(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn one_stdin() -> HeaderResult<()> {
    run_stdin(&[], ONE, "tests/expected/one.txt.out")
}

#[test]
fn one_n2_stdin() -> HeaderResult<()> {
    run_stdin(&["-n", "2"], ONE, "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4_stdin() -> HeaderResult<()> {
    run_stdin(&["-n", "4"], ONE, "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1_stdin() -> HeaderResult<()> {
    run_stdin(&["-c", "1"], ONE, "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2_stdin() -> HeaderResult<()> {
    run_stdin(&["-c", "2"], ONE, "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4_stdin() -> HeaderResult<()> {
    run_stdin(&["-c", "4"], ONE, "tests/expected/one.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn two() -> HeaderResult<()> {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n2() -> HeaderResult<()> {
    run(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4() -> HeaderResult<()> {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2() -> HeaderResult<()> {
    run(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4() -> HeaderResult<()> {
    run(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")
}

#[test]
fn two_stdin() -> HeaderResult<()> {
    run_stdin(&[], TWO, "tests/expected/two.txt.out")
}

#[test]
fn two_n2_stdin() -> HeaderResult<()> {
    run_stdin(&["-n", "2"], TWO, "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4_stdin() -> HeaderResult<()> {
    run_stdin(&["-n", "4"], TWO, "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2_stdin() -> HeaderResult<()> {
    run_stdin(&["-c", "2"], TWO, "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4_stdin() -> HeaderResult<()> {
    run_stdin(&["-c", "4"], TWO, "tests/expected/two.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn three() -> HeaderResult<()> {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n2() -> HeaderResult<()> {
    run(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4() -> HeaderResult<()> {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2() -> HeaderResult<()> {
    run(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4() -> HeaderResult<()> {
    run(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn three_stdin() -> HeaderResult<()> {
    run_stdin(&[], THREE, "tests/expected/three.txt.out")
}

#[test]
fn three_n2_stdin() -> HeaderResult<()> {
    run_stdin(&["-n", "2"], THREE, "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4_stdin() -> HeaderResult<()> {
    run_stdin(&["-n", "4"], THREE, "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2_stdin() -> HeaderResult<()> {
    run_stdin(&["-c", "2"], THREE, "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4_stdin() -> HeaderResult<()> {
    run_stdin(&["-c", "4"], THREE, "tests/expected/three.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn ten() -> HeaderResult<()> {
    run(&[TEN], "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2() -> HeaderResult<()> {
    run(&[TEN, "-n", "2"], "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4() -> HeaderResult<()> {
    run(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c2() -> HeaderResult<()> {
    run(&[TEN, "-c", "2"], "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4() -> HeaderResult<()> {
    run(&[TEN, "-c", "4"], "tests/expected/ten.txt.c4.out")
}

#[test]
fn ten_stdin() -> HeaderResult<()> {
    run_stdin(&[], TEN, "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2_stdin() -> HeaderResult<()> {
    run_stdin(&["-n", "2"], TEN, "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4_stdin() -> HeaderResult<()> {
    run_stdin(&["-n", "4"], TEN, "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c2_stdin() -> HeaderResult<()> {
    run_stdin(&["-c", "2"], TEN, "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4_stdin() -> HeaderResult<()> {
    run_stdin(&["-c", "4"], TEN, "tests/expected/ten.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn multiple_files() -> HeaderResult<()> {
    run(&[EMPTY, ONE, TWO, THREE, TEN], "tests/expected/all.out")
}

#[test]
fn multiple_files_n2() -> HeaderResult<()> {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-n", "2"],
        "tests/expected/all.n2.out",
    )
}

#[test]
fn multiple_files_n4() -> HeaderResult<()> {
    run(
        &["-n", "4", EMPTY, ONE, TWO, THREE, TEN],
        "tests/expected/all.n4.out",
    )
}

#[test]
fn multiple_files_c1() -> HeaderResult<()> {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-c", "1"],
        "tests/expected/all.c1.out",
    )
}

#[test]
fn multiple_files_c2() -> HeaderResult<()> {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-c", "2"],
        "tests/expected/all.c2.out",
    )
}

#[test]
fn multiple_files_c4() -> HeaderResult<()> {
    run(
        &["-c", "4", EMPTY, ONE, TWO, THREE, TEN],
        "tests/expected/all.c4.out",
    )
}
