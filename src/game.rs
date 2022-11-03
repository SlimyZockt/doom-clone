use crate::entity::Entity;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    Paused,
    GameOver,
}

struct Game {
    entities: vec<&mut impl Entity>
}
//TODO: implement functions
impl Game {
    fn start() {}
    fn add_entity(entity: Entity) {}
    fn remove_entity(i: usize) {}
    fn pause() {}
}