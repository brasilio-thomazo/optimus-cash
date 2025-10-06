const LIMIT: i32 = 10;

fn offset(page: i32) -> i32 {
    if page <= 1 {
        return 0;
    }
    (page - 1) * LIMIT
}

mod user_repository;
pub use user_repository::UserRepository;
