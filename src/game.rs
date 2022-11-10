use crate::entity::Entity;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    Paused,
    GameOver,
}

pub struct Game<T: Entity> {
    entities: Vec<T>
}
//TODO: implement functions
impl<T: Entity> Game<T> {
    fn start() {}
    fn add_entity(entity: impl Entity) {}
    fn remove_entity(i: usize) {}
    fn pause() {}
}