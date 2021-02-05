use crate::evaluator::alignment_error_rate;
use crate::evaluator::{AlgnGold, AlgnHard};
use crate::utils::cartesian_product;
use std::collections::HashSet;

pub enum AlgnMergeAction {
    JOIN, INTERSECT
}

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

pub fn join_algn(running: Option<Vec<AlgnHard>>, new: Vec<AlgnHard>) -> Option<Vec<AlgnHard>> {
    if let Some(running) = running {
        Some(
            running
                .iter()
                .zip(new)
                .map(|(old, new)| {
                    old.union(&new)
                        .copied()
                        .collect::<HashSet<(usize, usize)>>()
                })
                .collect::<Vec<HashSet<(usize, usize)>>>(),
        )
    } else {
        Some(new)
    }
}

pub fn params_to_alignment<T>(
    params: &[Vec<T>],
    extractors: &[(AlgnMergeAction, &dyn Fn(&[T]) -> Vec<AlgnHard>)],
) -> Vec<AlgnHard> {
    let mut running_algn: Option<Vec<AlgnHard>> = None;

    for (single_param, (merge_action, extractor)) in params.iter().zip(extractors.iter()) {
        let algn = extractor(single_param);
        running_algn = match merge_action {
            AlgnMergeAction::INTERSECT => intersect_algn(running_algn, algn),
            AlgnMergeAction::JOIN => join_algn(running_algn, algn),
        };
    }
    running_algn.unwrap()
}

pub fn gridsearch<T>(
    ranges: &[Vec<Vec<T>>],
    extractors: Vec<(AlgnMergeAction, &dyn Fn(&[T]) -> Vec<AlgnHard>)>,
    gold_algn: &[AlgnGold],
) -> (Vec<AlgnHard>, Vec<Vec<T>>, f32)
where
    T: Clone,
    T: std::fmt::Debug,
{
    // create linspace ranges
    let grid = cartesian_product(ranges);

    if grid.is_empty() {
        panic!("Empty list of parameters to optimize")
    }

    let mut min_aer = f32::INFINITY;
    let mut best_params: Option<Vec<Vec<T>>> = None;
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
