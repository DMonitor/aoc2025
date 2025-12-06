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

fn consolidate_range(mut range_set: &mut RangeSet) -> u32
{
    let mut changed = 0;
    let mut g:RangeSet = RangeSet::new();



    for new_range in range_set.iter()
    {
        let contains_lower = g.iter().find(|x| x.contains(new_range.start())).cloned();
        let contains_upper = g.iter().find(|x| x.contains(new_range.end())).cloned();

        if contains_lower.is_some() && contains_upper.is_some()
        {
            let s = *contains_lower.clone().unwrap().start();
            let e = *contains_upper.clone().unwrap().end();
            g.remove(&contains_upper.unwrap());
            g.remove(&contains_lower.unwrap());
            g.insert(RangeInclusive::new(s, e));
            changed += consolidate_range(&mut g);
        }
        else if contains_lower.is_some()
        {
            let s = *contains_lower.clone().unwrap().start();
            let e = u64::max(*new_range.end(),*contains_lower.clone().unwrap().end());
            g.remove(&contains_lower.unwrap());
            g.insert(RangeInclusive::new(s, e));
            changed += 1;
        }
        else if contains_upper.is_some()
        {
            let s = u64::min(*new_range.start(),*contains_upper.clone().unwrap().start());
            let e = *contains_upper.clone().unwrap().end();
            g.remove(&contains_upper.unwrap());
            g.insert(RangeInclusive::new(s,e));
            changed += 1;
        }
        else {
                        g.insert(new_range.clone());
        }
    }

    g.clone_into(range_set);
    if changed > 0
    {
        println!("changed {}", changed);
        return changed + consolidate_range(range_set);
    }

    0

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

    consolidate_range(&mut g);

    let mut x = g.into_iter().collect::<Vec<RangeInclusive<u64>>>();
    x.sort_by(|r1,r2| r1.start().cmp(r2.start()));

    let mut last_max = 0;

    for r in x
    {
        if(*r.start() < last_max)
        {
            println!(">>>{last_max} has a problem<<<")
        }
        last_max = *r.end();
        println!("{}-{}\t{}", r.start(), r.end(),r.clone().count());
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
    //println!("{}", solve_1());
    println!("{}", solve_2());
}