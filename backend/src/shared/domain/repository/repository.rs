use uuid::Uuid;

pub trait Repository<T> {
    fn find_all(&mut self) -> Vec<T>;
    fn find_by_id(&mut self, id: Uuid) -> Option<T>;
    fn save(&mut self, entity: T) -> Result<T, String>;
    fn delete(&mut self, id: Uuid) -> Result<(), String>;
}
