pub trait Repository<T> {
    fn find_all(&self) -> Vec<T>;
    fn find_by_id(&self, id: i32) -> Option<T>;
    fn save(&self, entity: T) -> Result<T, String>;
    fn delete(&self, id: i32) -> Result<(), String>;
}
