use itertools::Itertools;

// cba to parse my input today
const MIN_X: isize = 207;
const MAX_X: isize = 263;
const MIN_Y: isize = -115;
const MAX_Y: isize = -63;

pub fn day17(_input_lines: &[String]) -> (u64, u64) {
    // Okay, so part 1 today is really dumb.
    //
    // It's obvious that the X velocity is irrelevant - as long as there's an
    // initial X velocity where drag reduces X velocity to zero while inside
    // the target X range, and there trivially is, the highest trajectory
    // will be one where that is true and the probe drops into the target from
    // above.
    //
    // The trajectory that reaches the highest Y coordinate will be the one
    // that has the highest initial Y velocity. Since Y velocity reduces by
    // 1 each step, the probe will have exactly the same positive Y
    // coordinates on its way down as the ones it visited on the way up,
    // including Y=0. The highest positive starting Y velocity will also
    // therefore have the highest negative Y velocity at the point it reaches
    // Y=0, which is the highest one that *doesn't* overshoot the Y target
    // range on the very next step. If the bottom of the Y range were -100,
    // that means the probe must go from 0 to -100 in one step, so the
    // Y velocity in its *previous* step must be -99 - which means the initial
    // Y velocity must have been 99. The highest point it reaches would be the
    // 99th triangular number.
    let part1_y_velocity = MIN_Y.abs() - 1;
    let part1 = ((part1_y_velocity * (part1_y_velocity + 1)) / 2) as u64;

    // The minimum possible initial X velocity is one that reaches a point >=
    // MIN_X just as drag reduces it to 0 - which is to say, the index of the
    // first triangular number >= MIN_X. Very approximately, the square root
    // of 2 * MIN_X. The maximum possible X velocity is, of course, MAX_X.
    let min_x_velocity = ((MIN_X * 2) as f64).sqrt() as isize;
    let max_x_velocity = MAX_X;

    // We just calculated the maximum Y velocity, and the minimum Y velocity is
    // obviously MIN_Y.
    let min_y_velocity = MIN_Y;
    let max_y_velocity = part1 as isize;

    // This is a much bigger range than we really need to check, but it's
    // not that much computation.
    let part2 = (min_x_velocity..=max_x_velocity).cartesian_product(min_y_velocity..=max_y_velocity).filter(|(x, y)| Velocity { x: *x, y: *y }.reaches_target()).count() as u64;

    (part1, part2)
}

struct Velocity {
    x: isize,
    y: isize,
}

impl Velocity {
    fn reaches_target(mut self) -> bool {
        let mut x = 0isize;
        let mut y = 0isize;
        loop {
            x += self.x;
            y += self.y;
            self.x = std::cmp::max(0, self.x - 1);
            self.y -= 1;
            if x > MAX_X || y < MIN_Y {
                return false;
            }
            if (MIN_X..=MAX_X).contains(&x) && (MIN_Y..=MAX_Y).contains(&y) {
                return true;
            }
        }
    }
}