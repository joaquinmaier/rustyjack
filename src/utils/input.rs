pub fn handle_input( input: &String ) -> u8 {
    let chars: Vec<char> = String::as_str( input ).chars().into_iter().collect();

    return match chars[0] {
        'e' | 'E' => 1,
        's' => 2,
        'h' => 3,
        _ => 0
    }
}


