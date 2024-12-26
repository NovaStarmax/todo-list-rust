use crate::Status;
use crate::ToDo;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[cfg(test)]
mod tests {
    use super::*; // Importe les fonctions et structures du fichier.
    #[test]
    fn test_todo() {
        let antoine = ToDo {
            title: String::from("Antoine"),
            information: String::from("Gobbe"),
            status: Status::Done,
        };
        let antoine_bis = ToDo {
            title: String::from("Antoine"),
            information: String::from("Gobbe"),
            status: Status::Done,
        };

        assert_eq!(antoine, antoine_bis);
    }

    // const JSON: &str = r#"
    // {
    //     "title": "Antoine",
    //     "information": "Gobbe",
    //     "status": "Done"
    // }
    // "#;
    // #[test]
    // fn test_json() {
    //     let res = from_str::<ToDo>(JSON);
    //     println!("{:?}", res);
    // }

    // #[test]
    // fn is_err() {
    //     let err = 1;
    //     assert_eq!(err, 2);
    // }
    #[test]
    fn is_valid() {
        let valid = 1;
        assert_eq!(valid, 1);
    }
}
