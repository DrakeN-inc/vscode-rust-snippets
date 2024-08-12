# Rust snippets
## The Rust programming language snippets for Visual Studio Code

# Text [rust]:
## The simple text snippets
| Prefix:  | Description:                                                    |
| -------- | --------------------------------------------------------------- |
| hello    | println!("Hello, world!");  // DEBUG: "Hello, world!" message   |
| /TODO    | // TODO: ...                                                    |
| /NOTE    | // NOTE: ...                                                    |
| /DEBUG   | // DEBUG: ...                                                   |
| /FIXME   | // FIXME: ...                                                   |


# Attributes [rust]:
## The attributes snippets
| Prefix:                   | Description:                                 |
| ------------------------- | -------------------------------------------- |
| #[derive()]               | #[derive(...)]                               |
| #[allow()]                | #[allow(...)]                                |
| #[test]                   | #[test]                                      |
| #[cfg()]                  | #[cfg(...)]                                  |
| #[macro_use]              | #[macro_use]                                 |
| #[serde()]                | #[serde(...)]                                |
| #[proc_macro]             | #[proc_macro]                                |
| #[proc_macro_attribute]   | #[proc_macro_attribute]                      |
| #[proc_macro_derive()]    | #[proc_macro_derive(..., attributes(...))]   |


# Blocks [rust]:
## The blocks snippets
| Prefix:                 | Description:                           |
| ----------------------- | -------------------------------------- |
| mod {}                  | mod { ... }                            |
| struct {}               | struct ... { ... }                     |
| enum {}                 | enum ... { ... }                       |
| struct {} impl {}       | struct ... { ... }  impl ... { ... }   |
| enum {} impl {}         | enum ... { ... }  impl ... { ... }     |
| fn() {}                 | fn ...() { ... }                       |
| fn main() {}            | fn main() { ... }                      |
| async fn main() {}      | async fn main() { ... }                |
| macro_rules! {}         | macro_rules! ... { ... }               |
| if {}                   | if ... { ... }                         |
| if {} else {}           | if ... { ... } else { ... }            |
| if? {}                  | if ... { ... }else{ ... }              |
| match {}                | match ... { ... }                      |
| match Result<T, E> {}   | match Result<T, E> { ... }             |
| match Option<T> {}      | match Option<T> { ... }                |
| for {}                  | for v in ... { ... }                   |
| for (k, v) {}           | for (k, v) in ... { ... }              |
| while {}                | while ... { ... }                      |
| loop {}                 | loop { ... }                           |


# Operators [rust]:
## The code operators snippets
| Prefix:         | Description:                  |
| --------------- | ----------------------------- |
| use prelude::   | use crate::prelude::*;        |
| use crate::     | use crate::...;               |
| use super::     | use super::...;               |
| mod             | mod ...;                      |
| use             | use ...::...;                 |
| mod use         | mod ...;  pub use ...::...;   |
| let             | let ... = ...;                |
| static          | static ...:... = ...;         |
| const           | const ...:... = ...;          |
| return          | return ...;                   |
| break           | break ...;                    |
| break           | break;                        |
| continue        | continue;                     |


# Functions [rust]:
## The functions, methods && macros snippets
| Prefix:                | Description:              |
| ---------------------- | ------------------------- |
| .unwrap()              | .unwrap()                 |
| .unwrap_or()           | .unwrap_or(...)           |
| .unwrap_or_else()      | .unwrap_or_else(...)      |
| .expect()              | .expect(...)              |
| .map()                 | .map(...)                 |
| .map_err()             | .map_err(...)             |
| .to_lowercase()        | .to_lowercase()           |
| .to_uppercase()        | .to_uppercase()           |
| .split()               | .split(...)               |
| .rsplit()              | .rsplit(...)              |
| .splitn()              | .splitn(...)              |
| .rsplitn()             | .rsplitn(...)             |
| .split_once()          | .split_once(...)          |
| .rsplit_once()         | .rsplit_once(...)         |
| .split_whitespace()    | .split_whitespace(...)    |
| .split_terminator()    | .split_terminator(...)    |
| .rsplit_terminator()   | .rsplit_terminator(...)   |
| .replace()             | .replace(...)             |
| .replacen()            | .replacen(...)            |
| .repeat()              | .repeat(...)              |
| ::new()                | ::new(...)                |
| ::with_capacity()      | ::with_capacity(...)      |
| ::from()               | ::from(...)               |
| ::from_utf8()          | ::from_utf8(...)          |
| ::from_utf8_lossy()    | ::from_utf8_lossy(...)    |
| ::from_utf16()         | ::from_utf16(...)         |
| ::from_utf16_lossy()   | ::from_utf16_lossy(...)   |
| ::default()            | ::default()               |
| str!()                 | str!(...)                 |
| vec![]                 | vec![...]                 |
| deq![]                 | deq![...]                 |
| map!{}                 | map!{...}                 |
| set![]                 | set![...]                 |
| heap![]                | heap![...]                |
| list![]                | list![...]                |
| btree_map!{}           | btree_map!{...}           |
| btree_set![]           | btree_set![...]           |
| bson!{}                | bson!{...}                |
| rawbson!{}             | rawbson!{...}             |
| doc!{}                 | doc!{...}                 |
| rawdoc!{}              | rawdoc!{...}              |
| html!{}                | html!{...}                |
| json!{}                | json!{...}                |
| format!()              | format!(...)              |
| print!()               | print!(...)               |
| println!()             | println!(...)             |
| eprint!()              | eprint!(...)              |
| eprintln!()            | eprintln!(...)            |
| input!()               | input!(...)               |
| panic!()               | panic!(...)               |
| todo!()                | todo!(...)                |
| unimplemented!()       | unimplemented!(...)       |
| unreachable!()         | unreachable!(...)         |
