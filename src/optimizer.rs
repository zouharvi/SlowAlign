use crate::evaluator::alignment_error_rate;
use crate::evaluator::{AlgnGold, AlgnHard};
use crate::utils::cartesian_product;
use std::collections::HashSet;

fn join_to_running(running: Option<Vec<AlgnHard>>, new: Vec<AlgnHard>) -> Option<Vec<AlgnHard>> {
    if let Some(running) = running {
        Some(
            running
                .iter()
                .zip(new)
                .map(|(old, new)| {
                    old.intersection(&new)
                        .copied()
                        .collect::<HashSet<(usize, usize)>>()
                })
                .collect::<Vec<HashSet<(usize, usize)>>>(),
        )
    } else {
        Some(new)
    }
}

pub fn gridsearch(
    ranges: &[Vec<f32>],
    extractors: Vec<&dyn Fn(f32) -> Vec<AlgnHard>>,
    gold_algn: &[AlgnGold],
) -> (Vec<f32>, f32) {
    // create linspace ranges
    let grid = cartesian_product(ranges);

    if grid.is_empty() {
        panic!("Empty list of parameters to optimize")
    }

    let mut min_aer = f32::INFINITY;
    let mut best_params: Option<Vec<f32>> = None;

    for params in grid {
        println!("Trying {:?} ", params);
        assert_eq!(params.len(), extractors.len());

        let mut running_algn: Option<Vec<AlgnHard>> = None;

        for (single_param, extractor) in params.iter().zip(extractors.iter()) {
            let algn = extractor(*single_param);
            running_algn = join_to_running(running_algn, algn);
        }
        let aer = alignment_error_rate(&running_algn.unwrap(), gold_algn);
        println!("AER {}\n", aer);
        if aer < min_aer {
            best_params = Some(params);
            min_aer = aer;
        }
    }

    println!("Best AER: {}, {:?}", min_aer, best_params.clone().unwrap());

    (best_params.unwrap(), min_aer)
}
