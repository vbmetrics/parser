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

    let chars: Vec<char> = input.chars().collect();
    if chars.len() < 5 {
        warnings.push(ParserWarning { msg: "Za mało znaków dla zdarzenia specjalnego".to_string() });
        return Err(warnings);
    }

    let team: char = chars[0];
    if !['L', 'D', 'R'].contains(&team) {
        warnings.push(ParserWarning { msg: format!("Nieprawidłowa wartość TEAM: {}", team) });
    }

    let player: String = input[1..3].to_string();
    let skill: char = chars[3];
    let eval: char = chars[4];

    let modifier: Option<String> = if chars.len() > 5 { Some(chars[5].to_string()) } else { None };
    
    Ok((
        SpecialEvent {
            team,
            player,
            skill,
            eval,
            modifier,
        },
        warnings,
    ))
}

fn parse_normal_events(input: &str) -> (Vec<NormalEvent>, Vec<ParserWarning>) {
    let mut events = Vec::new();
    let mut warnings = Vec::new();
    let mut index: usize = 0;
    
    let size: usize = input.len();

    while index + 9 <= size {
        let chunk: &str = &input[index..index + 9];
        let chars: Vec<char> = chunk.chars().collect();

        let team: char = chars[0];
        if !['L', 'D'].contains(&team) {
            warnings.push(ParserWarning { msg: format!("Nieprawidłowa wartość TEAM: {}", team) });
        }

        let player: String = chunk[1..3].to_string();
        let skill: char = chars[3];
        let eval: char = chars[4];
        let start_zone: char = chars[5];
        let start_subzone: char = chars[6];
        let end_zone: char = chars[7];
        let end_subzone: char = chars[8];

        let mut modifier: Option<String> = None;
        
        index += 9;

        if index < size {
            let next_char: char = input.chars().nth(index).unwrap();
            if !['L', 'D'].contains(&next_char) {
                let mod_end: usize = if index + 1 < size { index + 2 } else { index + 1};
                modifier = Some(input[index..mod_end].to_string());
                index = mod_end;
            }
        }

        events.push(NormalEvent {
            team,
            player,
            skill,
            eval,
            start_zone,
            start_subzone,
            end_zone,
            end_subzone,
            modifier,
        });
    }

    (events, warnings)
}
