pub type Coordinate = (i64, i64);

pub fn text_to_button(input: &str) -> Coordinate {
    let next: String = input.chars().skip(10).collect();
    let entries: Vec<&str> = next.split(", ").collect();

    (
        text_to_coordinate(&entries.get(0).unwrap()),
        text_to_coordinate(&entries.get(1).unwrap()),
    )
}

pub fn text_to_coordinate(input: &str) -> i64 {
    input.chars().skip(2).collect::<String>().parse().unwrap()
}

pub fn text_to_prize(input: &str) -> Coordinate {
    let next: String = input.chars().skip(7).collect();
    let entries: Vec<&str> = next.split(", ").collect();

    (
        text_to_coordinate(&entries.get(0).unwrap()),
        text_to_coordinate(&entries.get(1).unwrap()),
    )
}
