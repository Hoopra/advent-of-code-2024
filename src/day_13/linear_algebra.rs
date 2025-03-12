use super::model::Coordinate;

pub fn solve_2n_order_system(a: &[Coordinate], c: &Coordinate) -> Coordinate {
    let (a1, b1) = a[0];
    let (a2, b2) = a[1];

    let (c1, c2) = c;

    let d_cb = c1 * b2 - b1 * c2;
    let d_ac = a1 * c2 - c1 * a2;
    let d_ab = a1 * b2 - b1 * a2;

    if d_ab == 0 || d_cb % d_ab != 0 || d_ac % d_ab != 0 {
        return (0, 0);
    }

    (d_cb / d_ab, d_ac / d_ab)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_2nd_order_system() {
        let a = [(12, 3), (2, -3)];
        let c = (15, 13);

        let result = solve_2n_order_system(&a, &c);
        assert_eq!(result, (2, -3));
    }
}
