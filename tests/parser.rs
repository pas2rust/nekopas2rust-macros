#![cfg(feature = "parser")]
use bincode::{Decode, Encode};
use nekopas2rust_macros::Parser;
#[derive(Parser, Decode, Encode, PartialEq, Eq, Debug, Clone)]
struct User {
    name: String,
    friend: Friend,
    age: u8,
}

#[derive(Parser, Encode, Debug, PartialEq, Eq, Hash, Decode, Clone)]
pub struct Friend {
    name: String,
}

#[test]
fn parser_hash_map() {
    let user = User {
        age: 18,
        friend: Friend {
            name: "John".to_string(),
        },
        name: "Abby".to_string(),
    };

    let user = user.to_hash_map();

    let name = user.get(&UserParserKey::UserName).unwrap();
    let age = user.get(&UserParserKey::UserAge).unwrap();
    let friend = user.get(&UserParserKey::UserFriend).unwrap();
    assert_eq!(name, &UserParserValue::UserName(String::from("Abby")));
    assert_eq!(age, &UserParserValue::UserAge(18));
    assert_eq!(
        friend,
        &UserParserValue::UserFriend(Friend {
            name: "John".to_string()
        })
    );
}

#[test]
fn parser_hash_set() {
    let user = User {
        age: 18,
        friend: Friend {
            name: "John".to_string(),
        },
        name: "Abby".to_string(),
    };

    let user = user.to_hash_set();

    let name = user
        .get(&UserParserValue::UserName(String::from("Abby")))
        .unwrap();
    let age = user.get(&UserParserValue::UserAge(18)).unwrap();
    let friend = user
        .get(&UserParserValue::UserFriend(Friend {
            name: "John".to_string(),
        }))
        .unwrap();

    assert_eq!(name, &UserParserValue::UserName(String::from("Abby")));
    assert_eq!(age, &UserParserValue::UserAge(18));
    assert_eq!(
        friend,
        &UserParserValue::UserFriend(Friend {
            name: "John".to_string()
        })
    );
}

#[test]
fn parser_bincode() {
    let user = User {
        age: 18,
        friend: Friend {
            name: "John".to_string(),
        },
        name: "Abby".to_string(),
    };

    let bytes = user.clone().to_bincode().unwrap();
    let decoded = User::from_bincode(bytes).unwrap();

    assert_eq!(user, decoded)
}
