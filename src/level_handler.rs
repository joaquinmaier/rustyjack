pub struct LevelHandler {
    pub level: i32,
    starting_bet: i32,
    progression: LevelProgressionSystem
}

impl LevelHandler
{
    pub fn new( starting_bet: i32, progression: LevelProgressionSystem ) -> LevelHandler {
        LevelHandler { level: 0, starting_bet, progression }
    }

    pub fn get_bet( &self ) -> f64 {
        return match self.progression {
            LevelProgressionSystem::Linear( multiplier )    => ( self.starting_bet as f64 * ( self.level as f64 * multiplier ) ).floor(),
            LevelProgressionSystem::Exponential( base )     => base.pow( self.level as u32 ) as f64
        }
    }

    pub fn increase_level( &mut self ) {
        self.level += 1;
    }

    pub fn peek_next_level_bet( &self ) -> f64 {
        let level = self.level + 1;

        return match self.progression {
            LevelProgressionSystem::Linear( multiplier )    => ( self.starting_bet as f64 * ( level as f64 * multiplier ) ).floor(),
            LevelProgressionSystem::Exponential( base )     => base.pow( level as u32 ) as f64
        }
    }
}

pub enum LevelProgressionSystem {
    Linear( f64 ),
    Exponential( i32 )
}
