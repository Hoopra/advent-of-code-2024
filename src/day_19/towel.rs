use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
};

use super::towel_stripe::TowelStripe;

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct Towel {
    stripes: Vec<TowelStripe>,
}

impl Debug for Towel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stripes: Vec<String> = self
            .stripes
            .iter()
            .map(|stripe| stripe.to_string())
            .collect();

        f.write_str(&format!("|{}|", &stripes.join("")))
    }
}

impl PartialOrd for Towel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.stripes.len().cmp(&other.stripes.len()))
    }
}

impl Ord for Towel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Towel {
    pub fn from_string(input: &str) -> Self {
        let stripes = input
            .chars()
            .map(|character| match character {
                'w' => TowelStripe::White,
                'u' => TowelStripe::Blue,
                'b' => TowelStripe::Black,
                'r' => TowelStripe::Red,
                _ => TowelStripe::Green,
            })
            .collect();

        Self { stripes }
    }

    pub fn many_from_string(input: &str, delimiter: &str) -> Vec<Towel> {
        input
            .split(delimiter)
            .map(|segment| Self::from_string(segment))
            .collect()
    }

    pub fn new(stripes: Vec<TowelStripe>) -> Self {
        Self { stripes }
    }
}

fn stripes_match(a: &[TowelStripe], b: &Vec<&TowelStripe>) -> bool {
    // if a.len() != b.len() {
    //     return false;
    // }

    a.iter().enumerate().all(|(i, stripe)| {
        let target = b.get(i);
        match target {
            Some(target) => target == &stripe,
            _ => true,
        }
    })
}

fn find_relevant<'a>(available: &[Towel], target: &[TowelStripe], skip: usize) -> Vec<Towel> {
    available
        .iter()
        .filter_map(|towel| {
            let target: Vec<&TowelStripe> = target.iter().skip(skip).collect();
            // target.iter().skip(skip).take(towel.stripes.len()).collect();

            match stripes_match(&towel.stripes, &target) {
                true => Some(towel.clone()),
                _ => None,
            }
        })
        .collect()
}

pub fn towel_combinations_from_string(input: &str) -> (Vec<Towel>, Vec<Towel>) {
    let mut components = input.split("\n\n");

    let available = Towel::many_from_string(components.nth(0).unwrap(), ", ");
    let combinations = Towel::many_from_string(components.nth(0).unwrap(), "\n");

    (available, combinations)
}

impl Towel {
    pub fn is_combination_possible(&self, available: &Vec<Towel>) -> bool {
        let Towel {
            stripes: desired, ..
        } = self;

        let mut checked: HashSet<Towel> = HashSet::new();

        let start_nodes = find_relevant(available, &desired, 0);
        let mut queue: BinaryHeap<Towel> = BinaryHeap::from(start_nodes);

        while let Some(next) = queue.pop() {
            let Towel { stripes } = &next;
            let relevant = find_relevant(available, &desired, stripes.len());

            // if relevant.len() == 0 {
            //     checked.insert(next.clone());
            // }

            for towel in relevant {
                if checked.contains(&towel) {
                    continue;
                }

                let mut stripes = stripes.clone();
                // println!("stripes.len before {} / {}", stripes.len(), desired.len());
                stripes.extend(towel.stripes.clone());
                // println!("stripes.len after {} / {}", stripes.len(), desired.len());

                match stripes.len().cmp(&desired.len()) {
                    Ordering::Equal => return true,
                    Ordering::Greater => {
                        // println!("too many stripes!");
                        checked.insert(towel.clone());
                    }
                    Ordering::Less => {
                        // println!("too few stripes!");
                    }
                }

                queue.push(Towel::new(stripes));
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_if_combination_is_possible() {
        let input =
            "uwbr, bbgrb, wwbg, gugurur, rgurw, rbgw, bw, wurbru, bgrurw, gbw, bwgg, ubwg, wrgb, bwg, rwgug, bbgb, grbrbuug, ruw, grrrbu, ubwurbwu, rbg, buu, bru, rwugubg, brgwu, wr, uwb, wgwu, rubgwu, rrw, bwwrb, rrr, uww, gbgwrww, gbb, uugw, wurgr, wgbwbrur, grwbg, gwrbuw, wgu, wrr, ug, rbr, guur, gurru, urgwggw, wuwbwg, bwggrwgw, brurww, wbr, buww, bgrb, bbb, rgr, bgbbwbu, bububub, wbugu, gwgub, gubbwwrb, rrgwuuwu, uggb, rgbg, ubgbrw, rrrg, rr, gbuwb, wwbugwb, rbwbr, wbrrb, rug, b, bwwrwr, gug, bwrw, rbu, r, bgu, uub, bwb, ubwuuw, uuur, wgur, urw, gbbw, uuuru, ur, wburb, gwuuw, rbb, wuu, gru, urb, wb, bbwb, grgugw, rgruuur, rrwbwrg, uubru, ubrw, rbbgrgr, br, wub, ww, guu, gur, uug, bww, ubrwbgu, brwu, wrbgub, ggwwgwb, uuwuugrb, rwgrw, ggurr, gbg, rrwb, gbrbgr, bbruu, rrbrbw, wrurug, ruuw, wg, bwwwwb, bubg, ubr, bwu, uugub, ruuuubb, rggw, urr, urwgr, rww, wwggu, gbbwgw, ubggu, wbb, uugr, uubgu, uwg, rwwuru, uuurb, buuu, wbru, ubwr, urg, urrwrwgb, rbubb, uw, brrg, bgb, gwrguu, bgwu, bubwbg, rb, grb, uugubb, gww, bggbru, gwb, grwwwugb, gwggbg, bbg, ubrg, rgu, gwguu, ruwrwb, gggwb, bbbbug, ruwrurbw, rrbww, rwbww, grru, wwrwwub, guwurb, ugwbg, wgruuwwb, rg, uubgww, ruu, gbbbrbb, ggruu, gwgbwu, buw, uur, uuurbgr, uu, gwwrwgb, brrr, uwuw, guw, rwwr, gwuguub, wgb, brwwr, brub, uurg, ugb, rguub, brwbg, ruuuu, bggbgw, wrgwwwg, rrgw, uurrg, wwuwu, gugg, wwub, wuw, uuu, wugruu, rrgb, uwr, rbrwwb, grr, rrrb, bgg, ruwr, ggrurwr, ugg, bwr, rguwru, wgub, ugr, wrwu, gwbw, bwbur, wgbbr, rrrw, grwrgg, rgb, ugu, wwg, wugur, ggu, ubrwwgbu, uwub, urubwug, ugw, wguub, wbubugw, gugwwu, rwb, ubuu, gbu, wugbg, gbbu, rrg, brbr, urrg, guwub, bbr, wbbuburr, wwburwb, grbugbu, uuwbrw, wubr, wbbggww, ubub, ruru, grwbgbb, bbw, uru, wgg, ubrwu, rw, wurbu, uwrwb, rrb, rugggr, rgrwr, wrw, ggw, grgugu, uwug, rur, wruuw, bugbur, bb, uwggrrb, gbgr, rgbrw, bur, rbw, bub, wrb, rrgr, wrug, gr, uubw, gbr, rguuwbb, buubg, ugrbrwr, gubu, ru, wgr, ubggb, gwuu, rwgru, rwwrwg, grrr, gwu, guuuwww, ggugug, ubwrgbww, rgwbggu, brr, uubbr, wgwbw, ub, rrbru, uwuuur, rgwr, rruw, urgguw, urwb, uggr, bu, brwrw, gggub, uurb, buuw, ggb, wbrbgu, gurb, wuwbu, grg, buubbg, ggrubb, urwbwrb, uwugrgu, rurrw, ubw, wrg, u, www, bgr, wrbgw, wbg, rub, wwu, ubrgwg, bwuw, brb, ggr, bwrb, wggb, ubb, wrrrgw, urubrgwg, bbgr, uwwr, rwg, buwubw, bgw, brg, w, rubb, bwwwbb, ggg, bbbw, gbguw, guuw, grggr, wur, wug, grwwgrw, wbbwu, rru, bgbwgr, brw, gwg, uubwwur, wrwgw, wubwb, ugubrb, burb, ggwrr, rwbg, bwgugw, gbgwgwgr, uwgb, wu, gugww, wwuugrwb, rugu, wgw, rwub, uuwwbg, grrg, uuw, gu, gwgb, uwwur, rwbbbbb, gbur, uwugrwgg, rrgg, buwg, rwr, bwuwrgb, ubu, bgbrguwb, rbru, gw, bbbbg, ubuwuu, bbu, rgg, rwu, wbw, ggwbru, guwb, wwr, wuubg, ubrww, rwruru, urbww, gwrr, wrgwbu, bwbrugu, wwrww, grw, rgw, wwb, wguuwrw, wrgrg, wgug, uwwg, bbubwbru, wrwbu, gub, bwbg, wgrwr, gg, burbrbw, uggu, rgbgbw, gwr, rgrwbw, gguuur, urbw, wwrgw, wrggu, gbug, ubug\n\nwgrggwgruubbgbgurwrbgggwbuwwruburwrbrwwgrubg";

        let (available, combinations) = towel_combinations_from_string(input);

        let result = combinations
            .first()
            .unwrap()
            .is_combination_possible(&available);
        assert_eq!(result, false);
    }
}
