use std::collections::HashSet;
use std::ops::RangeInclusive;


const INPUT0: &str = include_str!("../res/input0");
const INPUT1: &str = include_str!("../res/input1");

type RangeSet = HashSet<RangeInclusive<u64>>;


fn get_fresh() -> RangeSet
{
    INPUT0.trim()
        .lines()
        .into_iter()
        .map( |line| {
            let range = line.split_at(line.find('-').unwrap());
            RangeInclusive::new(
                range.0.parse::<u64>().unwrap(),
                range.1[1..].parse::<u64>().unwrap()
            )
        }).collect::<RangeSet>()
}

fn get_pantry() -> Vec<u64>
{
    INPUT1
        .trim()
        .lines()
        .map(|s|
            s.parse::<u64>().unwrap()).collect::<Vec<u64>>()
}

fn consolidate_range(range_set: &mut RangeSet)
{
    let mut g:RangeSet = RangeSet::new();
    for new_range in range_set.iter()
    {
        let contains_lower = g.iter().find(|x| x.contains(new_range.start())).cloned();
        let contains_upper = g.iter().find(|x| x.contains(new_range.end())).cloned();

        match (contains_lower, contains_upper)
        {
            (Some(lower), Some(upper)) => {
                let s = *lower.start();
                let e = *upper.end();
                g.remove(&upper);
                g.remove(&lower);
                g.insert(RangeInclusive::new(s, e));
                consolidate_range(&mut g);
            }
            (Some(lower), None) => {
                let s = *lower.start();
                let e = u64::max(*new_range.end(),*lower.end());
                g.remove(&lower);
                g.insert(RangeInclusive::new(s, e));
            }
            (None, Some(upper)) => {
                let s = u64::min(*new_range.start(),*upper.start());
                let e = *upper.end();
                g.remove(&upper);
                g.insert(RangeInclusive::new(s,e));
            }
            (None, None) => {
                g.insert(new_range.clone());
            }
        }
    }

    g.clone_into(range_set);

}


fn solve_2() -> u64 {

    // build new range set g []
    // to add range R:
    //   - find range L that contains lower bound
    //   - find range U that contains upper bound
    // if U and L exist, merge them
    // if only L exists, change its upper bound
    // if only U exists, change its lower bound

    let mut g = get_fresh();

    consolidate_range(&mut g);

    let mut sum = 0;

    for r in g
    {
        sum += r.clone().count()
    }

    sum as u64
}

fn solve_1() -> u64 {

    let good_ingredients = get_fresh();
    get_pantry()
        .iter()
        .filter(|i|
            good_ingredients
                .iter()
                .any(|r| r.contains(i)))
        .count() as u64
}

fn main()
{
    println!("{}", solve_1());
    println!("{}", solve_2());
}