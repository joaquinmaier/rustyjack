pub fn handle_input( input: &String ) -> u8 {
    let chars: Vec<char> = String::as_str( input ).chars().into_iter().collect();

    return match chars[0] {
        'e' | 'E'   => 1,
        's'         => 2,
        'h'         => 3,
        'S'         => 4,
        'd'         => 5,
        'H'         => 6,
        _           => 0
    }
}

pub fn wants_to_insure( input: &String ) -> Option<bool> {
    let chars: Vec<char> = String::as_str( input ).chars().into_iter().collect();

    return match chars[0] {
        'y' | 'Y' | 'i' | 'I'   => Some(true),
        'n' | 'N'               => Some(false),
        _                       => None
    }
}

