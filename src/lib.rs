use pgx::*;

pg_module_magic!();

/// Apply ROT13 to `text`.
#[pg_extern]
fn rot13(mut text: String) -> String {
    // Safety: pgx has requested that Postgres allocate/free
    //         a string on our behalf during the request. The
    //         String is safe to mutate because we have ownership.
    unsafe {
        for byte in text.as_bytes_mut() {
            *byte = match *byte {
                b'a'..=b'm' | b'A'..=b'M' => *byte + 13,
                b'n'..=b'z' | b'N'..=b'Z' => *byte - 13,
                _ => continue,
            }
        }
    }
    text
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_pg_rot13() {
        let cases = [
            ("hello", "uryyb"),
            ("HELLO", "URYYB"),
            ("HELLO", "URYYB"),
            ("Grüß Gott", "Teüß Tbgg"),
        ];

        for (input, output) in cases.iter() {
            let text = String::from(*input);
            let expected_output = String::from(*output);
            let actual_output = crate::rot13(text);
            assert_eq!(expected_output, actual_output);
        }
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
