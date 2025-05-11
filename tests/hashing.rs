// use crate::lib::hashing::Argon2id;

// #[cfg(test)]
// mod test {
//     use crate::lib::hashing::Argon2id;

//     #[test]
//     fn test_hashing_and_verification_works_as_expected() {
//         let password = "P@ssw0rd".to_string();

//         let hash = Argon2id::hash_password(&password).expect("hash_password returned an error");

//         let is_match =
//             Argon2id::verify_password(&password, &hash).expect("verify_password returned an error");

//         assert!(is_match);
//     }
// }
