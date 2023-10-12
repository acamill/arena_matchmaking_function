use crate::*;

pub struct ContainerParams {
    pub program_id: Pubkey,
    pub user: Pubkey,
    pub realm_pda: Pubkey,
    pub user_account_pda: Pubkey,
    pub spaceship_pda: Pubkey,
    pub faction: u8,
    pub opponent_spaceship_1_pda: Pubkey,
    pub opponent_spaceship_2_pda: Pubkey,
    pub opponent_spaceship_3_pda: Pubkey,
    pub opponent_spaceship_4_pda: Pubkey,
    pub opponent_spaceship_5_pda: Pubkey,
}

impl ContainerParams {
    pub fn decode(container_params: &Vec<u8>) -> std::result::Result<Self, SwitchboardError> {
        let params = String::from_utf8(container_params.clone()).unwrap();

        let mut program_id: Pubkey = Pubkey::default();
        let mut user: Pubkey = Pubkey::default();
        let mut realm_pda: Pubkey = Pubkey::default();
        let mut user_account_pda: Pubkey = Pubkey::default();
        let mut spaceship_pda: Pubkey = Pubkey::default();
        let mut faction: u8 = 0;
        let mut opponent_spaceship_1_pda: Pubkey = Pubkey::default();
        let mut opponent_spaceship_2_pda: Pubkey = Pubkey::default();
        let mut opponent_spaceship_3_pda: Pubkey = Pubkey::default();
        let mut opponent_spaceship_4_pda: Pubkey = Pubkey::default();
        let mut opponent_spaceship_5_pda: Pubkey = Pubkey::default();

        for env_pair in params.split(',') {
            let pair: Vec<&str> = env_pair.splitn(2, '=').collect();
            if pair.len() == 2 {
                match pair[0] {
                    "PID" => program_id = Pubkey::from_str(pair[1]).unwrap(),
                    "USER" => user = Pubkey::from_str(pair[1]).unwrap(),
                    "REALM_PDA" => realm_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "USER_ACCOUNT_PDA" => user_account_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "SPACESHIP_PDA" => spaceship_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "FACTION" => faction = pair[1].parse::<u8>().unwrap(),
                    "OS_1_PDA" => opponent_spaceship_1_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "OS_2_PDA" => opponent_spaceship_2_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "OS_3_PDA" => opponent_spaceship_3_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "OS_4_PDA" => opponent_spaceship_4_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "OS_5_PDA" => opponent_spaceship_5_pda = Pubkey::from_str(pair[1]).unwrap(),
                    _ => {}
                }
            }
        }

        if program_id == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }
        if user == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }
        if realm_pda == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }
        if user_account_pda == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }
        if spaceship_pda == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }
        if opponent_spaceship_1_pda == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }
        if opponent_spaceship_2_pda == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }
        if opponent_spaceship_3_pda == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }
        if opponent_spaceship_4_pda == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }
        if opponent_spaceship_5_pda == Pubkey::default() {
            return Err(SwitchboardError::InvalidFunctionInput);
        }

        Ok(Self {
            program_id,
            user,
            realm_pda,
            user_account_pda,
            spaceship_pda,
            faction,
            opponent_spaceship_1_pda,
            opponent_spaceship_2_pda,
            opponent_spaceship_3_pda,
            opponent_spaceship_4_pda,
            opponent_spaceship_5_pda,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_decode() {
        let request_params_string = format!(
            "PID={},USER={},REALM_PDA={},USER_ACCOUNT_PDA={},SPACESHIP_PDA={},FACTION={},OS_1_PDA={},OS_2_PDA={},OS_3_PDA={},OS_4_PDA={},OS_5_PDA={}",
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            1,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
        );
        let request_params_bytes = request_params_string.into_bytes();

        let params = ContainerParams::decode(&request_params_bytes).unwrap();

        assert_eq!(params.program_id, anchor_spl::token::ID);
        assert_eq!(params.user, anchor_spl::token::ID);
        assert_eq!(params.realm_pda, anchor_spl::token::ID);
        assert_eq!(params.user_account_pda, anchor_spl::token::ID);
        assert_eq!(params.spaceship_pda, anchor_spl::token::ID);
        assert_eq!(params.faction, 1);
        assert_eq!(params.opponent_spaceship_1_pda, anchor_spl::token::ID);
        assert_eq!(params.opponent_spaceship_2_pda, anchor_spl::token::ID);
        assert_eq!(params.opponent_spaceship_3_pda, anchor_spl::token::ID);
        assert_eq!(params.opponent_spaceship_4_pda, anchor_spl::token::ID);
        assert_eq!(params.opponent_spaceship_5_pda, anchor_spl::token::ID);
    }
}
