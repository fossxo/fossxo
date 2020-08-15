use amethyst::{
    core::{shrev::EventChannel, timing::Time},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, Write},
};

use crate::components::*;
use crate::events::PlayerEvent;
use crate::resources::*;

/// Generates player events for the AI opponents.
#[derive(SystemDesc)]
pub struct AiPlayerSystem;

impl<'a> System<'a> for AiPlayerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, AiPlayer>,
        Read<'a, GameData>,
        Read<'a, Time>,
        Write<'a, EventChannel<PlayerEvent>>,
    );

    fn run(&mut self, (players, ai_players, game_data, time, mut channel): Self::SystemData) {
        for (player, ai_player) in (&players, &ai_players).join() {
            // Check to see if is the player's turn, if not skip the player so CPU cycles are not
            // spent evaluating positions that will not be used.
            if game_data.is_players_move(player)
                && sufficient_delay_since_last_move(ai_player, &game_data, &time)
            {
                if let Some(position) = ai_player.ai_opponent.get_move(&game_data.game) {
                    channel.single_write(PlayerEvent::RequestMark(*player, position));
                }
            }
        }
    }
}

// Indicates if sufficient time has elapsed since the lsat move.
fn sufficient_delay_since_last_move(
    ai_player: &AiPlayer,
    game_data: &GameData,
    time: &Time,
) -> bool {
    time.absolute_time() - game_data.last_move_time >= ai_player.move_delay
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_ttt_lib as ttt;

    #[test]
    fn sufficient_delay_since_last_move_when_same_delay_should_return_true() {
        let mut ai_player = AiPlayer::new(ttt::ai::Difficulty::None);
        let time = Time::default();
        let mut game_data = GameData::default();

        let zero = core::time::Duration::from_secs(0);
        assert_eq!(time.absolute_time(), zero);
        ai_player.move_delay = zero;
        game_data.last_move_time = zero;

        let result = sufficient_delay_since_last_move(&ai_player, &game_data, &time);

        assert_eq!(result, true);
    }

    #[test]
    fn sufficient_delay_since_last_move_when_less_delay_should_return_false() {
        let mut ai_player = AiPlayer::new(ttt::ai::Difficulty::None);
        let mut time = Time::default();
        let mut game_data = GameData::default();

        ai_player.move_delay = core::time::Duration::from_secs(5);
        time.set_delta_seconds(2.0);
        game_data.last_move_time = core::time::Duration::from_secs(0);

        let result = sufficient_delay_since_last_move(&ai_player, &game_data, &time);

        assert_eq!(result, false);
    }
}
