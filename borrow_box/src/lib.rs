#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(Self {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }
    pub fn read_winner(&self) -> (String, u16) {
        // we have a box of game session?
        let game_sesh = &self.clone();
        // TO:DO FIND-WHY DE-REFERENCING ONCE.

        if game_sesh.p1.1 > game_sesh.p2.1 {
            return (game_sesh.p1.0.clone(), game_sesh.p1.1);
        } else if game_sesh.p1.1 < game_sesh.p2.1 {
            return (game_sesh.p2.0.clone(), game_sesh.p2.1);
        }
        ("Same score! tied".to_string(), game_sesh.p1.1)

        // if number of games minus p1/p2

        //lets find mid point
        // if nb games odd --> plus 1 divide by 2
        // else nb games even -->
        // if nb_games = 5
        // let mid_point = (game_sesh.nb_games / 2) + 1;
    }
    pub fn update_score(&mut self, user_name: String) {
        let mid_point = (self.nb_games / 2) + 1;
        if self.p1.1<mid_point && self.p2.1<mid_point{
            match user_name {
                player_1 if self.p1.0==player_1 => self.p1.1+=1,
                _player_2 if self.p2.0==user_name => self.p2.1+=1,
                _=>(),
            }
        }
    }
    pub fn delete(self) -> String {
        let game=self;
        let deleted_game=String::from(format!("game deleted: id -> {}",game.id));
        drop(game);
        deleted_game
    }
}
