use crate::ToDo;
use crate::Status;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*; // Importe les fonctions et structures du fichier.
    #[test]
    fn test_todo() {
        let antoine = ToDo{
            title: String::from("Antoine"),
            information: String::from("Gobbe"),
            status: Status::Done,
        };
        let antoine_bis = ToDo{
            title: String::from("Antoine"),
            information: String::from("Gobbe"),
            status: Status::Done,
        };
        
        assert_eq!(antoine, antoine_bis);
    }

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