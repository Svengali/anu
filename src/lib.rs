#![allow(dead_code)]

mod com;
//mod ent;
mod sys;



pub fn print( str: String ) {
    println!( "Test {}", str );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
