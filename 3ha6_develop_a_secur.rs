// API Specification for Secure Game Prototype Simulator

mod game_logic {
    pub struct Game {
        pub id: usize,
        pub players: Vec<Player>,
        pub game_state: GameState,
    }

    pub struct Player {
        pub id: usize,
        pub username: String,
        pub password: String, // hashed password for security
    }

    pub enum GameState {
        NotStarted,
        InProgress,
        Finished,
    }

    impl Game {
        pub fn new(id: usize) -> Self {
            Game {
                id,
                players: Vec::new(),
                game_state: GameState::NotStarted,
            }
        }

        pub fn add_player(&mut self, player: Player) {
            self.players.push(player);
        }

        pub fn start_game(&mut self) {
            self.game_state = GameState::InProgress;
        }

        pub fn finish_game(&mut self) {
            self.game_state = GameState::Finished;
        }
    }
}

mod security {
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;

    pub struct SecureDataStore {
        pub store: HashMap<String, String>,
    }

    impl SecureDataStore {
        pub fn new() -> Self {
            SecureDataStore {
                store: HashMap::new(),
            }
        }

        pub fn store_data(&mut self, key: &str, value: &str) {
            let mut hasher = Sha256::new();
            hasher.input_str(key);
            let hashed_key = hasher.result_str();
            self.store.insert(hashed_key, value.to_string());
        }

        pub fn retrieve_data(&self, key: &str) -> Option<String> {
            let mut hasher = Sha256::new();
            hasher.input_str(key);
            let hashed_key = hasher.result_str();
            self.store.get(&hashed_key).cloned()
        }
    }
}

mod api {
    use game_logic::{Game, Player, GameState};
    use security::SecureDataStore;

    pub struct SecureGameAPIService {
        pub games: Vec<Game>,
        pub secure_data_store: SecureDataStore,
    }

    impl SecureGameAPIService {
        pub fn new() -> Self {
            SecureGameAPIService {
                games: Vec::new(),
                secure_data_store: SecureDataStore::new(),
            }
        }

        pub fn create_game(&mut self) -> &Game {
            let new_game = Game::new(self.games.len());
            self.games.push(new_game.clone());
            new_game
        }

        pub fn add_player_to_game(&mut self, game_id: usize, player: Player) {
            let game = self.games.iter_mut().find(|g| g.id == game_id).unwrap();
            game.add_player(player);
        }

        pub fn start_game(&mut self, game_id: usize) {
            let game = self.games.iter_mut().find(|g| g.id == game_id).unwrap();
            game.start_game();
        }

        pub fn finish_game(&mut self, game_id: usize) {
            let game = self.games.iter_mut().find(|g| g.id == game_id).unwrap();
            game.finish_game();
        }

        pub fn store_secure_data(&mut self, key: &str, value: &str) {
            self.secure_data_store.store_data(key, value);
        }

        pub fn retrieve_secure_data(&self, key: &str) -> Option<String> {
            self.secure_data_store.retrieve_data(key)
        }
    }
}