use crate::validation::{validate_team, validate_player, validate_skill, validate_eval, validate_zone, validate_subzone};
use crate::translation::tr;

#[derive(Debug)]
pub struct ParserWarning {
    pub msg: String,
}

#[derive(Debug)]
pub enum Event {
    Normal(NormalEvent),
    Special(SpecialEvent),
}

#[derive(Debug)]
pub struct NormalEvent {
    pub team: char,
    pub player: String,
    pub skill: char,
    pub eval: char,
    pub start_zone: char,
    pub start_subzone: char,
    pub end_zone: char,
    pub end_subzone: char,
    pub modifier: Option<String>,
}

#[derive(Debug)]
pub struct SpecialEvent {
    pub team: char,
    pub player: String,
    pub skill: char,
    pub eval: char,
    pub modifier: Option<String>
}

#[derive(Debug)]
pub struct ParsedAction {
    pub events: Vec<Event>,
    pub comment: Option<String>,
    pub warnings: Vec<ParserWarning>
}

pub fn parse_line(line: &str) -> ParsedAction {
    let mut warnings: Vec<ParserWarning> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    let mut comment: Option<String> = None;

    // separate comment with code
    let parts: Vec<&str> = line.splitn(2, '%').collect();
    let data_part = parts[0].trim();
    if parts.len() > 1 {
        comment = Some(parts[1].trim().to_string());
    }

    // check if special event
    if data_part.starts_with('!') {
        match parse_special_event(&data_part[1..]) {
            Ok((event, mut ev_warnings)) => {
                events.push(Event::Special(event));
                warnings.append(&mut ev_warnings);
            }
            Err(err_warnings) => {
                warnings.extend(err_warnings);
            }
        }
    } else {
        let (normal_events, norm_warnings) = parse_normal_events(data_part);
        events.extend(normal_events.into_iter().map(Event::Normal));
        warnings.extend(norm_warnings);
    }

    ParsedAction {
        events,
        comment,
        warnings,
    }
}

fn parse_special_event(input: &str) -> Result<(SpecialEvent, Vec<ParserWarning>), Vec<ParserWarning>> {
    let mut warnings: Vec<ParserWarning> = Vec::new();

    let size: usize = input.len();

    if size < 5 {
        warnings.push(ParserWarning { msg: format!("{}", tr("Not enough characters")) });
        return Err(warnings);
    }

    let team: char = input.chars().nth(0).unwrap();
    let player: &str = &input[1..3];
    let skill: char = input.chars().nth(3).unwrap();
    let eval: char = input.chars().nth(4).unwrap();
    let modifier: Option<String> = if size > 5 { Some(input[5..].to_string()) } else { None };

    if let Some(w) = validate_team(team, true) { warnings.push(ParserWarning { msg: w }); }
    if let Some(w) = validate_player(player) { warnings.push(ParserWarning { msg: w }); }
    if let Some(w) = validate_skill(skill, true) { warnings.push(ParserWarning { msg: w }); }
    if let Some(w) = validate_eval(eval, true) { warnings.push(ParserWarning { msg: w }); }

    Ok((
        SpecialEvent {
            team,
            player: player.to_string(),
            skill,
            eval,
            modifier,
        },
        warnings,
    ))
}

fn parse_normal_events(input: &str) -> (Vec<NormalEvent>, Vec<ParserWarning>) {
    let mut events: Vec<NormalEvent> = Vec::new();
    let mut warnings: Vec<ParserWarning> = Vec::new();
    let mut index: usize = 0;
    
    let size: usize = input.len();

    /* if size < 9 {
        warnings.push(ParserWarning { msg: format!("{}", tr("Not enough characters")) });
        return (events, warnings);
    } */

    while index < size {
        if !input[index..].starts_with(['L', 'O']) {
            warnings.push(ParserWarning { msg: format!("{}: {}", tr("Unexpected character"), &input[index..index + 1]) });
            index += 1;
            continue;
        }

        if index + 9 > size {
            warnings.push(ParserWarning { msg: format!("{}", tr("Not enough characters for full event")) });
            break;
        }

        let chunk: &str = &input[index..index + 9];
        let chars: Vec<char> = chunk.chars().collect();

        if let Some(w) = validate_team(chars[0], false) { warnings.push(ParserWarning { msg: w }); }
        if let Some(w) = validate_player(&chunk[1..3]) { warnings.push(ParserWarning { msg: w }); }
        if let Some(w) = validate_skill(chars[3], false) { warnings.push(ParserWarning { msg: w }); }
        if let Some(w) = validate_eval(chars[4], false) { warnings.push(ParserWarning { msg: w }); }
        if let Some(w) = validate_zone(chars[5]) { warnings.push(ParserWarning { msg: w }); }
        if let Some(w) = validate_subzone(chars[6]) { warnings.push(ParserWarning { msg: w }); }
        if let Some(w) = validate_zone(chars[7]) { warnings.push(ParserWarning { msg: w }); }
        if let Some(w) = validate_subzone(chars[8]) { warnings.push(ParserWarning { msg: w }); }

        let mut modifier: Option<String> = None;
        let mut next_index = index + 9;

        /* let modifier: Option<String> = if index + 9 < size {
            let next_char = input.chars().nth(index + 9).unwrap();
            if !['L', 'D'].contains(&next_char) {
                Some(input[index + 9..].to_string())
            } else {
                None
            }
        } else {
            None
        }; */

        //let modifier_len: usize = modifier.as_ref().map_or(0, |m| m.len());

        while next_index < size && !input[next_index..].starts_with(['L', 'O']) {
            next_index += 1;
        }

        if next_index > index + 9 {
            modifier = Some(input[index + 9..next_index].to_string());
        }

        events.push(NormalEvent {
            team: chars[0],
            player: chunk[1..3].to_string(),
            skill: chars[3],
            eval: chars[4],
            start_zone: chars[5],
            start_subzone: chars[6],
            end_zone: chars[7],
            end_subzone: chars[8],
            modifier,
        });

        // index += 9 + modifier_len;
        index = next_index;
    }

    (events, warnings)
}
