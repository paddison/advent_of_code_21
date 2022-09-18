static BOARD_SIZE: usize = 10;
static COUNTS_PER_ROLL: [usize; 7] = [1, 3, 6, 7, 6, 3, 1]; 

fn get_input() -> Vec<Player> {
    include_str!("../data/day_21.txt")
        .split('\n')
        .map(|l| l[l.find(':').unwrap() + 1..].trim().parse::<usize>().unwrap().into())
        .collect()
}

pub fn get_solution_1() -> usize {
    let players = get_input();
    let (losing_score, turns) = play(players, Dice::default());
    losing_score * turns
}

pub fn get_solution_2() -> usize {
    let players = get_input();
    let mut prev_wins = 0;
    let mut cur_wins = 0;
    play_dirac(
        players[0], 
        players[1], 
        &mut cur_wins,
        &mut prev_wins, 
        1);
    std::cmp::max(prev_wins, cur_wins)
}

#[derive(Clone, Copy)]
struct Player {
    score: usize,
    position: usize,
}

impl From<usize> for Player {
    fn from(position: usize) -> Self {
        Player { score: 0, position }
    }
}

fn play(mut players: Vec<Player>, mut dice: Dice) -> (usize, usize) {
    let n_players = players.len();

    loop {
        let player = &mut players[dice.count % n_players];
        do_turn(player, dice.roll());
        
        if player.score >= 1000 {
            break (players[(dice.count) % n_players].score, dice.count)
        }
        
    }
}

fn do_turn(player: &mut Player, roll: usize) {
    let new_pos = ((player.position + roll - 1) % BOARD_SIZE) + 1;
    player.position = new_pos;
    player.score += new_pos;
}

struct Dice {
    count: usize,
    sides: usize,
}

impl Dice {
    fn roll(&mut self) -> usize {
        self.count += 3;
        (self.count - 2) % self.sides +
        (self.count - 1) % self.sides +
        (self.count - 0) % self.sides
    }
}
impl Default for Dice {
    fn default() -> Self {
        Dice { count: 0, sides: 100 }
    }
}

fn play_dirac(current_player: Player, previous_player: Player, current_player_wins: &mut usize, previous_player_wins: &mut usize, universe_count: usize) {
    if previous_player.score >= 21 {
        *previous_player_wins += universe_count;
    } else {
        for score in 3..10 {
            play_dirac(previous_player, do_dirac_turn(current_player, score), previous_player_wins, current_player_wins, universe_count * COUNTS_PER_ROLL[score - 3]); // 1 1 1 = 3
        }
    }

}

fn do_dirac_turn(mut player: Player, roll: usize) -> Player {
    let new_pos = ((player.position + roll - 1) % BOARD_SIZE) + 1;
    player.position = new_pos;
    player.score += new_pos;
    player
}


#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<Player> {
        include_str!("../data/day_21_test.txt")
            .split('\n')
            .map(|l| l[l.find(':').unwrap() + 1..].trim().parse::<usize>().unwrap().into())
            .collect()
    }

    #[test]
    fn test_roll_dice() {
        let mut d = Dice::default();
        assert_eq!(d.roll(), 6);
        assert_eq!(d.count, 3);

        assert_eq!(d.roll(), 15);
        assert_eq!(d.count, 6);

        assert_eq!(d.roll(), 24);
        assert_eq!(d.count, 9);

    }

    #[test]
    fn test_players_do_turn() {
        let mut players = get_test_input();
        let mut dice = Dice::default();

        do_turn(&mut players[0], dice.roll());
        assert_eq!(players[0].position, 10);
        assert_eq!(players[0].score, 10);

        do_turn(&mut players[1], dice.roll());
        assert_eq!(players[1].position, 3);
        assert_eq!(players[1].score, 3);

        do_turn(&mut players[0], dice.roll());
        assert_eq!(players[0].position, 4);
        assert_eq!(players[0].score, 14);

        do_turn(&mut players[1], dice.roll());
        assert_eq!(players[1].position, 6);
        assert_eq!(players[1].score, 9);

        do_turn(&mut players[0], dice.roll());
        assert_eq!(players[0].position, 6);
        assert_eq!(players[0].score, 20);

        do_turn(&mut players[1], dice.roll());
        assert_eq!(players[1].position, 7);
        assert_eq!(players[1].score, 16);

        do_turn(&mut players[0], dice.roll());
        assert_eq!(players[0].position, 6);
        assert_eq!(players[0].score, 26);

        do_turn(&mut players[1], dice.roll());
        assert_eq!(players[1].position, 6);
        assert_eq!(players[1].score, 22);
    }
    
    #[test]
    fn test_play_test_data() {
        let players = get_test_input();
        let (losing_score, turns) = play(players, Dice::default());
        assert_eq!(turns, 993);
        assert_eq!(losing_score, 745);
    }

    #[test]
    fn test_play_test_data_part_2() {
        let players = get_test_input();
        let mut prev_wins = 0;
        let mut cur_wins = 0;
        play_dirac(
            players[0], 
            players[1], 
            &mut prev_wins, 
            &mut cur_wins,
            1);
        assert_eq!(std::cmp::max(prev_wins, cur_wins), 444356092776315);
    }
}