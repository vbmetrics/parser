#[cfg(test)]
mod tests {
    use parser::parser::{parse_line, Event};

    #[test]
    fn test_normal_event_parsing() {
        let result: parser::parser::ParsedAction = parse_line("L07S+1A5BD02R+5B3C%Test parsera");
        assert_eq!(result.events.len(), 2);
        assert_eq!(result.comment, Some("Test parsera".to_string()));
    }

    #[test]
    fn test_special_event_parsing() {
        let result = parse_line("!D00H+");
        assert_eq!(result.events.len(), 1);
        if let Event::Special(event) = &result.events[0] {
            assert_eq!(event.team, 'D');
            assert_eq!(event.player, "00");
            assert_eq!(event.skill, 'H');
            assert_eq!(event.eval, '+');
        }
    }
}
