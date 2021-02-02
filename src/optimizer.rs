use crate::evaluator::alignment_error_rate;
use crate::evaluator::{AlgnGold, AlgnHard, AlgnSoft};
use std::collections::HashSet;

fn cartesian_product<T>(lists: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut res = vec![];
    let mut lists = lists.iter();
    // handle empty array case
    if let Some(first_list) = lists.next() {
        for i in first_list.clone() {
            res.push(vec![i]);
        }
    }
    for l in lists {
        let mut next_out = vec![];
        for r in res {
            for el in l {
                let mut new_tuple = r.clone();
                new_tuple.push(el.clone());
                next_out.push(new_tuple);
            }
        }
        res = next_out;
    }
    res
}

fn join_to_running(running: Option<AlgnHard>, new: AlgnHard) -> Option<AlgnHard> {
    if running.is_none() {
        Some(new)
    } else {
        Some(
            running
                .unwrap()
                .iter()
                .zip(new)
                .map(|(old, new)| {
                    old.intersection(&new)
                        .map(|x| x.clone())
                        .collect::<HashSet<(usize, usize)>>()
                })
                .collect::<Vec<HashSet<(usize, usize)>>>(),
        )
    }
}

pub fn gridsearch(
    ranges: &[(f32, f32, usize)],
    extractors: &[&dyn Fn(f32) -> AlgnHard],
    gold_algn: &AlgnGold,
) -> (Option<Vec<f32>>, f32) {
    // create linspace ranges
    let ranges = ranges
        .iter()
        .map(|(start, end, steps)| {
            (0..*steps)
                .map(|step| {
                    (end - start) * (step as f32) / {
                        if *steps <= 1 {
                            panic!("Number of steps in gridsearch has to be at least 2")
                        }
                        *steps as f32 - 1.0
                    } + start
                })
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();
    let grid = cartesian_product(&ranges);

    if grid.is_empty() {
        panic!("Empty list of parameters to optimize")
    }

    let mut min_aer = f32::INFINITY;
    let mut best_param: Option<Vec<f32>> = None;

    for params in grid {
        println!("Trying {:?} ", params);
        assert_eq!(params.len(), extractors.len());

        let mut running_algn: Option<AlgnHard> = None;

        for (single_param, extractor) in params.iter().zip(extractors) {
            let algn = extractor(*single_param);
            running_algn = join_to_running(running_algn, algn);
        }
        let aer = alignment_error_rate(running_algn.unwrap(), gold_algn);
        println!("AER {}\n", aer);
        if aer < min_aer {
            best_param = Some(params);
            min_aer = aer;
        }
    }

    (best_param, min_aer)
}
