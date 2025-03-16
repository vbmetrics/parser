use crate::translation::tr;

pub fn validate_team(team: char, special: bool) -> Option<String> {
    let mut valid_team_chars: Vec<char> = vec!['L', 'O'];

    if special {
        valid_team_chars.push('R');
    }

    if !valid_team_chars.contains(&team) {
        Some(format!("{}: {}", tr("Invalid team value"), team))
    } else {
        None
    }
}

pub fn validate_player(player: &str) -> Option<String> {
    if player.len() != 2 || !player.chars().all(|c: char| c.is_numeric()) {
        Some(format!("{}: {}", tr("Invalid player's number"), player))
    } else {
        None
    }
}

pub fn validate_skill(skill: char, special: bool) -> Option<String> {
    let valid_skills: &[char] = if special {
        &['F', 'T', 'H', 'C']
    } else {
        &['S', 'R', 'P', 'A', 'B', 'D']
    };

    if !valid_skills.contains(&skill) {
        Some(format!("{}: {}", tr("Invalid skill type"), skill))
    } else {
        None
    }
}

pub fn validate_eval(eval: char, special: bool) -> Option<String> {
    let valid_evals = if special { ['=', '+', '-'] } else { ['+', '-', '#'] };

    if !valid_evals.contains(&eval) {
        Some(format!("{}: {}", tr("Invalid eval type"), eval))
    } else {
        None
    }
}

pub fn validate_zone(zone: char) -> Option<String> {
    if !('1'..='9').contains(&zone) {
        Some(format!("{}: {}", tr("Invalid zone"), zone))
    } else {
        None
    }
}

pub fn validate_subzone(subzone: char) -> Option<String> {
    if !('A'..='I').contains(&subzone) {
        Some(format!("{}: {}", tr("Invalid subzone"), subzone))
    } else {
        None
    }
}
