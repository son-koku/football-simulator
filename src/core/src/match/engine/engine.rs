use crate::r#match::ball::Ball;
use crate::r#match::field::MatchField;
use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::squad::TeamSquad;
use crate::r#match::{FootballMatchResult, GameState, MatchPlayer, MatchState};
use nalgebra::Vector3;

pub struct FootballEngine<const W: usize, const H: usize> {}

impl<const W: usize, const H: usize> FootballEngine<W, H> {
    pub fn play(home_squad: TeamSquad, away_squad: TeamSquad) -> FootballMatchResult {
        let mut context = MatchContext::new(FieldSize::new(W, H));

        let mut field = MatchField::new(W, H);

        field.setup(home_squad, away_squad);

        context.result.write_team_players(
            field.home_players.as_ref().unwrap(),
            field.away_players.as_ref().unwrap(),
        );

        // First half
        context.state.set_game_state(GameState::FirstHalf);
        Self::play_inner(&mut field, &mut context);

        {
            field.swap_squads();
            field.swap_player_positions();

            Self::play_rest_time(&mut field);
        }

        // Second half
        context.state.set_game_state(GameState::SecondHalf);
        Self::play_inner(&mut field, &mut context);

        if context.result.additinal_time_ms > 0 {
            // additional time
            context.state.set_game_state(GameState::ExtraTime);

            Self::play_inner(&mut field, &mut context);
        }

        context.result
    }

    fn play_rest_time(field: &mut MatchField) {
        field.players.iter_mut().for_each(|p| {
            p.player_attributes.rest(1000);
        })
    }

    fn play_inner(field: &mut MatchField, context: &mut MatchContext) -> u64 {
        let mut additional_time: u64 = 0;

        while context.increment_time() {
            let ball_update_events = field.ball.update(context);

            // handle ball
            Ball::handle_events(context.time.time, ball_update_events, context);

            // setup positions
            let objects_positions = MatchObjectsPositions::from(&field);

            let player_update_events = field
                .players
                .iter_mut()
                .flat_map(|player| {
                    player.update(context.time.time, &context.state, &objects_positions)
                })
                .collect();

            // handle player
            MatchPlayer::handle_events(player_update_events, context);

            field.write_match_positions(&mut context.result, context.time.time);
        }

        additional_time
    }
}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}

pub struct MatchContext {
    pub state: MatchState,
    time: MatchTime,
    pub result: FootballMatchResult,
    pub field_size: FieldSize,
}

impl MatchContext {
    pub fn new(field_size: FieldSize) -> Self {
        MatchContext {
            state: MatchState::new(),
            time: MatchTime::new(),
            result: FootballMatchResult::with_match_time(MATCH_HALF_TIME_MS),
            field_size,
        }
    }

    pub fn increment_time(&mut self) -> bool {
        self.time.increment(MATCH_TIME_INCREMENT_MS) < MATCH_HALF_TIME_MS
    }
}

pub struct FieldSize {
    pub width: usize,
    pub height: usize,
}

impl FieldSize {
    pub fn new(width: usize, height: usize) -> Self {
        FieldSize { width, height }
    }
}

const MATCH_TIME_INCREMENT_MS: u64 = 10;
const MATCH_HALF_TIME_MS: u64 = 1 * 60 * 1000;

pub struct MatchTime {
    pub time: u64,
}

impl MatchTime {
    pub fn new() -> Self {
        MatchTime { time: 0 }
    }

    pub fn increment(&mut self, val: u64) -> u64 {
        self.time += val;
        self.time
    }
}

pub struct MatchObjectsPositions {
    pub ball_positions: Vector3<f32>,
    pub players_positions: Vec<PlayerFieldPosition>,
}

impl MatchObjectsPositions {
    pub fn from(field: &MatchField) -> Self {
        MatchObjectsPositions {
            ball_positions: field.ball.position,
            players_positions: field
                .players
                .iter()
                .map(|p| PlayerFieldPosition {
                    player_id: p.player_id,
                    is_home: p.is_home,
                    position: p.position,
                })
                .collect(),
        }
    }

    fn find_closest_teammate(
        &self,
        current_player: &MatchPlayer,
        state: &MatchState,
    ) -> Option<Vector3<f32>> {
        let max_pass_distance = 30.0;

        let mut closest_teammate = None;
        let mut closest_distance = f32::MAX;

        for teammate_player_position in self.players_positions.iter() {
            if teammate_player_position.player_id == current_player.player_id {
                continue;
            }

            if teammate_player_position.is_home != current_player.is_home {
                continue;
            }

            let distance = current_player
                .position
                .distance_to(&teammate_player_position.position);

            if distance < closest_distance && distance < max_pass_distance {
                closest_teammate = Some(teammate_player_position.position);
                closest_distance = distance;
            }
        }

        closest_teammate
    }
}
