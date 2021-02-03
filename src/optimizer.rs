use crate::evaluator::alignment_error_rate;
use crate::evaluator::{AlgnGold, AlgnHard};
use crate::utils::cartesian_product;
use std::collections::HashSet;

pub fn intersect_algn(running: Option<Vec<AlgnHard>>, new: Vec<AlgnHard>) -> Option<Vec<AlgnHard>> {
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

pub fn params_to_alignment(
    params: &[f32],
    extractors: &[&dyn Fn(f32) -> Vec<AlgnHard>],
) -> Vec<AlgnHard> {
    let mut running_algn: Option<Vec<AlgnHard>> = None;

    for (single_param, extractor) in params.iter().zip(extractors.iter()) {
        let algn = extractor(*single_param);
        running_algn = intersect_algn(running_algn, algn);
    }
    running_algn.unwrap()
}

pub fn gridsearch(
    ranges: &[Vec<f32>],
    extractors: Vec<&dyn Fn(f32) -> Vec<AlgnHard>>,
    gold_algn: &[AlgnGold],
) -> (Vec<AlgnHard>, Vec<f32>, f32) {
    // create linspace ranges
    let grid = cartesian_product(ranges);

    if grid.is_empty() {
        panic!("Empty list of parameters to optimize")
    }

    let mut min_aer = f32::INFINITY;
    let mut best_params: Option<Vec<f32>> = None;
    let mut best_algn: Option<Vec<AlgnHard>> = None;

    for params in grid {
        eprintln!("Trying {:?} ", params);
        assert_eq!(params.len(), extractors.len());

        let algn = params_to_alignment(&params, &extractors);
        let aer = alignment_error_rate(&algn, gold_algn);
        eprintln!("AER {}, best {}\n", aer, min_aer);
        if aer < min_aer {
            best_params = Some(params);
            min_aer = aer;
            best_algn = Some(algn);
        }
    }

    eprintln!("Best AER: {}, {:?}", min_aer, best_params.clone().unwrap());

    (best_algn.unwrap(), best_params.unwrap(), min_aer)
}
