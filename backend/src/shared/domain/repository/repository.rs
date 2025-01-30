use uuid::Uuid;

pub trait Repository<T> {
    fn find_all(&mut self) -> Result<Vec<T>, String>;
    fn find_by_id(&mut self, id: Uuid) -> Result<Option<T>, String>;
    fn save(&mut self, entity: T) -> Result<T, String>;
    fn delete(&mut self, id: Uuid) -> Result<(), String>;
}
