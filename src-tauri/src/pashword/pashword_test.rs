use regex::Regex;

use crate::pashword;

#[test]
fn generated_pashword_always_reproducible() {
    let result1 = pashword::generate_pashword(
        "{\"website\":\"reddit.com\",\"username\":\"asda1111\",\"password\":\"asdasd1\"}",
        20,
        "reddit.com",
        "asda1111",
    );
    assert_eq!(result1, "&a3SZ5e$99m%ZK9rN*_T");

    let result2 = pashword::generate_pashword(
        "{\"website\":\"reddit.com\",\"username\":\"asdasda\",\"password\":\"asasdasd\"}",
        11,
        "reddit.com",
        "asdasda",
    );
    assert_eq!(result2, "fAp.&5Ri82#");

    let result3 = pashword::generate_pashword(
        "{\"website\":\"asjdajsd.com\",\"username\":\"asdiuhas9h321940u23u9rb2i230\",\"password\":\"sdihjc8sydc 23474 8234 90-24\"}",
        11,
        "asjdajsd.com",
        "asdiuhas9h321940u23u9rb2i230",
    );
    assert_eq!(result3, "2_Rz_W8iseA");

    let result4 = pashword::generate_pashword(
        "{\"website\":\"asjdajsd.com\",\"username\":\"asdiuhas9h321940u23u9rb2i230\",\"password\":\"sdihjc8sydc 23474 8234 90-24\"}",
        15,
        "asjdajsd.com",
        "asdiuhas9h321940u23u9rb2i230",
    );
    assert_eq!(result4, "H_ex3WOq*eA2a_T");

    let result5 = pashword::generate_pashword(
        "{\"website\":\"asjdajsd.com\",\"username\":\"asdiuhas9h321940u23u9rb2i230\",\"password\":\"sdihjc8sydc 23474 8234 90-24\"}",
        20,
        "asjdajsd.com",
        "asdiuhas9h321940u23u9rb2i230",
    );
    assert_eq!(result5, "20_xMWOiseA2a_TdvpZL");
}

#[test]
fn pashword_contains_number_symbol_alphabet_only() {
    // Using multiple regexes instead of '(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[@#$%&*\._!])'
    // as rust doesn't support lookahead in regexes
    let re1 = Regex::new(r"^[a-zA-Z0-9@#$%&*\._!]+$").unwrap();
    let re2 = Regex::new(r"^.*[a-z].*$").unwrap();
    let re3 = Regex::new(r"^.*[A-Z].*$").unwrap();
    let re4 = Regex::new(r"^.*[0-9].*$").unwrap();
    let re5 = Regex::new(r"^.*[@#$%&*\._!].*$").unwrap();
    fn match_regex(
        pashword: &str,
        re1: &Regex,
        re2: &Regex,
        re3: &Regex,
        re4: &Regex,
        re5: &Regex,
    ) -> bool {
        re1.is_match(pashword)
            && re2.is_match(pashword)
            && re3.is_match(pashword)
            && re4.is_match(pashword)
            && re5.is_match(pashword)
    }

    let result1 = pashword::generate_pashword(
        "{\"website\":\"a.com\",\"username\":\"a\",\"password\":\"a\"}",
        20,
        "a.com",
        "a",
    );
    assert!(match_regex(&result1, &re1, &re2, &re3, &re4, &re5));

    let result2 = pashword::generate_pashword(
        "{\"website\":\"a.com\",\"username\":\"a1\",\"password\":\"a\"}",
        20,
        "a.com",
        "a1",
    );
    assert!(match_regex(&result2, &re1, &re2, &re3, &re4, &re5));
    assert!(match_regex("aBca1.", &re1, &re2, &re3, &re4, &re5));
    assert!(!match_regex("123a-+", &re1, &re2, &re3, &re4, &re5));
    assert!(!match_regex("123", &re1, &re2, &re3, &re4, &re5));
}

#[test]
fn sanitize_password_works() {
    // Using multiple regexes instead of '(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[@#$%&*\._!])'
    // as rust doesn't support lookahead in regexes
    let re1 = Regex::new(r"^[a-zA-Z0-9@#$%&*\._!]+$").unwrap();
    let re2 = Regex::new(r"^.*[a-z].*$").unwrap();
    let re3 = Regex::new(r"^.*[A-Z].*$").unwrap();
    let re4 = Regex::new(r"^.*[0-9].*$").unwrap();
    let re5 = Regex::new(r"^.*[@#$%&*\._!].*$").unwrap();
    fn match_regex(
        pashword: &str,
        re1: &Regex,
        re2: &Regex,
        re3: &Regex,
        re4: &Regex,
        re5: &Regex,
        no_symbols: bool,
        no_numbers: bool,
    ) -> bool {
        let res1 = re1.is_match(pashword);
        let res2 = re2.is_match(pashword);
        let res3 = re3.is_match(pashword);
        let mut res4 = re4.is_match(pashword);
        let mut res5 = re5.is_match(pashword);
        if no_symbols {
            res5 = !res5;
        }
        if no_numbers {
            res4 = !res4;
        }
        return res1 && res2 && res3 && res4 && res5;
    }

    assert!(match_regex(
        "20_xMWOiseA2a_TdvpZL",
        &re1,
        &re2,
        &re3,
        &re4,
        &re5,
        false,
        false
    ));

    let result1 = pashword::sanitize("20_xMWOiseA2a_TdvpZL", true, false);
    assert!(match_regex(
        &result1, &re1, &re2, &re3, &re4, &re5, true, false
    ));

    let result2 = pashword::sanitize("20_xMWOiseA2a_TdvpZL", false, true);
    assert!(match_regex(
        &result2, &re1, &re2, &re3, &re4, &re5, false, true
    ));

    let result3 = pashword::sanitize("20_xMWOiseA2a_TdvpZL", true, true);
    assert!(match_regex(
        &result3, &re1, &re2, &re3, &re4, &re5, true, true
    ));
}
