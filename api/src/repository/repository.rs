use crate::config;

pub trait Repository {
    fn offset(&self, page: i32) -> i32 {
        let limit = self.limit();
        if page <= 1 {
            return 0;
        }
        (page - 1) * limit
    }

    fn limit(&self) -> i32 {
        config::get_var_or("PAGINATION_LIMIT", 10)
    }
}
