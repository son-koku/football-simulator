use crate::shared::fullname::FullName;
use crate::context::GlobalContext;
use crate::utils::{DateUtils, Logging};
use chrono::NaiveDate;
use std::fmt::{Display, Formatter, Result};
use crate::club::{StaffClubContract, StaffResult, 
                  StaffPosition, PersonBehaviour};
use crate::{Relations, StaffCollectionResult};

#[derive(Debug)]
pub struct Staff {
    pub id: u32,
    pub full_name: FullName,
    pub birth_date: NaiveDate,
    pub behaviour: PersonBehaviour,

    pub contract: Option<StaffClubContract>,

    pub relations: Relations
}

impl Staff {
    pub fn new(
        id: u32,
        full_name: FullName,
        birth_date: NaiveDate,
        contract: Option<StaffClubContract>,
    ) -> Self {
        Staff {
            id,
            full_name,
            birth_date,
            contract,
            behaviour: PersonBehaviour::default(),
            relations: Relations::new(),
        }
    }

    pub fn stub() -> Self {
        Staff {
            id: 0,
            full_name: FullName {
                first_name: "stub".to_string(),
                last_name: "stub".to_string(),
                middle_name: "stub".to_string(),
            },
            contract: None,
            behaviour: PersonBehaviour::default(),
            birth_date: NaiveDate::from_ymd(2019, 1, 1),
            relations: Relations::new(),
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> StaffResult {
        let result = StaffResult::new();
        
        if DateUtils::is_birthday(self.birth_date, ctx.simulation.date.date()) {}

        result  
    }
}

//DISPLAY
impl Display for Staff {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.full_name, self.birth_date)
    }
}

#[derive(Debug)]
pub struct StaffCollection {
    pub staffs: Vec<Staff>,
    pub roles: StaffRoles,

    stub: Staff,
}

#[derive(Debug)]
pub struct StaffRoles {
    main_coach: Option<StaffClubContract>,
    contract_resolver: Option<StaffClubContract>,
}

impl StaffCollection {
    pub fn new(staffs: Vec<Staff>) -> Self {
        StaffCollection {
            staffs,
            roles: StaffRoles {
                main_coach: None,
                contract_resolver: None,
            },
            stub: Staff::stub(),
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> StaffCollectionResult {
        let staff_results = self.staffs
            .iter_mut()
            .map(|staff| {
                let message = &format!("simulate staff: id: {}", &staff.id);
                Logging::estimate_result(
                    || staff.simulate(ctx.with_staff(Some(staff.id))),
                    message,
                )
            })
            .collect();
        
        return StaffCollectionResult::new(staff_results);
    }

    pub fn main_coach(&self) -> &Staff {
        let main_coach = self.get_by_position(StaffPosition::Coach);
        match main_coach.first() {
            Some(coach) => {
                &coach
            },
            None => {
                &self.stub
            }
        }
    }

    pub fn coaches(&self) -> Vec<&Staff> {
        self.get_by_position(StaffPosition::Coach)
    }

    pub fn contract_resolver(&self) -> &Staff {
        *self.get_by_position(StaffPosition::Coach).first().unwrap()
    }

    fn get_by_position(&self, position: StaffPosition) -> Vec<&Staff> {
        let staffs: Vec<&Staff> = self.staffs.iter().filter(|staff| {
            staff.contract.is_some() && staff.contract.as_ref().unwrap().position == position
        }).collect();

        if staffs.is_empty() {
            return vec![&self.stub];
        }

        staffs
    }
}
