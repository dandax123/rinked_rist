#[cfg(test)]
// pub mod first;
// pub mod second;
pub mod third;
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn main() {
    println!(
        "The size of a default string is  {}",
        std::mem::size_of::<Vec<String>>()
    )
}
