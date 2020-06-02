use crate::club::Club;
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDate;
use crate::league::LeagueSettings;

#[derive(Debug)]
pub struct ScheduleManager {
    pub items: Vec<ScheduleItem>,
    pub current_tour: Option<Tour>
}

#[derive(Debug, Clone)]
pub struct ScheduleItem {
    pub date: NaiveDateTime,
    pub home_club_id: u32,
    pub guest_club_id: u32,
}

#[derive(Debug)]
pub struct Tour {
    pub games: Vec<ScheduleItem>
}

impl ScheduleManager {
    pub fn new() -> Self {
        ScheduleManager {
            items: Vec::new(),
            current_tour: None
        }
    }
    
    pub fn is_schedule_exists(&self) -> bool {
        !self.items.is_empty()
    }
    
    pub fn start_new_tour(&mut self, date: NaiveDate){
        let mut current_week_games = Vec::with_capacity(30);
        
        for day in 0..7 {
            let filter_date = date + Duration::days(day);

            for day_game in self.items.iter().filter(|s| s.date.date() == filter_date) {
                current_week_games.push(day_game.clone())
            }           
        }       
        
        self.current_tour = Some(Tour::new(current_week_games));
    }
    
    fn get_nearest_saturday(current_date: NaiveDate, league_settings: &LeagueSettings) -> NaiveDate {
        let (start_day, start_month) = league_settings.season_starting;
        
        let mut current_date = NaiveDate::from_ymd(
            current_date.year(), start_month as u32, start_day as u32);
        
        loop {
            if current_date.weekday() == Weekday::Sat {
                break;
            }

            current_date += Duration::days(1)
        }

        current_date
    }
    
    fn generate_for_day(club_ids: &[Club], count: u8, date: NaiveDate) -> Vec<ScheduleItem> {
        let mut res = Vec::with_capacity(count as usize);
        
        
        
        res
    }
    
    pub fn generate(&mut self, current_date: NaiveDate, clubs: &[Club], league_settings: &LeagueSettings) {
        let club_len = clubs.len();

        let club_len_half: u8 = (club_len / 2) as u8;
  
        let mut schedule_items = Vec::with_capacity(club_len * 2);

        let mut current_date = ScheduleManager::get_nearest_saturday(current_date, league_settings);
        
        let mut rng = &mut rand::thread_rng();
        //
        // loop {
        //     if current_date == league_settings.season_ending {
        //         break;
        //     }
        //    
        //     let saturday = starting_date;
        //     let sunday = starting_date + Duration::days(1);
        //
        //     for item in Self::generate_for_day(clubs, club_len_half, saturday) {
        //         schedule_items.push(item);
        //     }
        //
        //     for item in Self::generate_for_day(clubs, club_len_half, sunday) {
        //         schedule_items.push(item);
        //     }
        //
        //     current_date += Duration::days(1);
        // }
        
        self.items = schedule_items;
        self.current_tour = None;
    }

    pub fn get_matches(&self, date: NaiveDateTime) -> Vec<&ScheduleItem> {
        self.items.iter().filter(|x| x.date == date).collect()
    }
}

impl Tour {
    pub fn new(games: Vec<ScheduleItem>) -> Self {
        Tour {
            games
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::club::ClubMood;
    use crate::people::{PlayerCollection, StaffCollection};
    use crate::ClubBoard;

    #[test]
    fn generate_is_correct() {
        //let mut clubs = Vec::new();
        //
        // clubs.push(Club {
        //     id: 1,
        //     name: "1".to_string(),
        //     mood: ClubMood::default(),
        //     board: ClubBoard::new(),
        //     players: PlayerCollection::new(vec![]),
        //     staffs: StaffCollection::new(vec![]),
        //     tactics: None,
        //     transfer_list: vec![],
        //     match_history: vec![]
        // });
        //
        // clubs.push(Club {
        //     id: 2,
        //     name: "1".to_string(),
        //     mood: ClubMood::default(),
        //     board: ClubBoard::new(),
        //     players: PlayerCollection::new(vec![]),
        //     staffs: StaffCollection::new(vec![]),
        //     tactics: None,
        //     transfer_list: vec![],
        //     match_history: vec![]
        // });
        //
        // clubs.push(Club {
        //     id: 3,
        //     name: "1".to_string(),
        //     mood: ClubMood::default(),
        //     board: ClubBoard::new(),
        //     players: PlayerCollection::new(vec![]),
        //     staffs: StaffCollection::new(vec![]),
        //     tactics: None,
        //     transfer_list: vec![],
        //     match_history: vec![]
        // });
        //
        // let schedule = Schedule::generate(&clubs, NaiveDate::from_ymd(2020, 3, 1)).unwrap();

        //sassert_eq!(2, schedule.items.len());
    }
}
