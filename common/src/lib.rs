pub mod api;
pub mod model;
pub mod utils;

#[cfg(test)]
mod tests {
    // use super::*;

    #[tokio::test]
    async fn it_works() {
        assert_eq!(true, true);
    }
}
