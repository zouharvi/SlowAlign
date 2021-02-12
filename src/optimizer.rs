use crate::evaluator::{alignment_error_rate, alignment_reverse};
use crate::evaluator::{AlgnGold, AlgnHard, AlgnSoft};
use crate::linspace;
use crate::pack;
use crate::utils::cartesian_product;
use crate::{align_hard, align_soft};
use std::collections::HashSet;

#[derive(Copy, Clone)]
pub enum AlgnMergeAction {
    JOIN,
    INTERSECT,
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
    package: &AlignmentPackage,
    extractors: &[(
        AlgnMergeAction,
        &dyn Fn(&[T], &AlignmentPackage) -> Vec<AlgnHard>,
    )],
) -> Vec<AlgnHard> {
    let mut running_algn: Option<Vec<AlgnHard>> = None;

    for (single_param, (merge_action, extractor)) in params.iter().zip(extractors.iter()) {
        let algn = extractor(single_param, package);
        running_algn = match merge_action {
            AlgnMergeAction::INTERSECT => intersect_algn(running_algn, algn),
            AlgnMergeAction::JOIN => join_algn(running_algn, algn),
        };
    }
    running_algn.unwrap()
}

pub fn gridsearch<T>(
    package: &AlignmentPackage,
    extractors: &[(
        Vec<Vec<T>>,
        AlgnMergeAction,
        &dyn Fn(&[T], &AlignmentPackage) -> Vec<AlgnHard>,
    )],
    gold_algn: &[AlgnGold],
) -> (Vec<AlgnHard>, Vec<Vec<T>>, f32)
where
    T: Clone,
    T: std::fmt::Debug,
    T: std::marker::Sized,
{
    // create linspace ranges
    let ranges = extractors
        .iter()
        .map(|(range, _action, _func)| range.clone())
        .collect();
    let functions = extractors
        .iter()
        .map(|(_range, action, func)| (*action, *func))
        .collect::<Vec<(
            AlgnMergeAction,
            &dyn Fn(&[T], &AlignmentPackage) -> Vec<AlgnHard>,
        )>>();
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

        let algn = params_to_alignment(&params, package, &functions);
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

pub struct AlignmentPackage<'a> {
    pub alignment_fwd: &'a [AlgnSoft],
    pub alignment_rev: &'a [AlgnSoft],
    pub alignment_diag: &'a [AlgnSoft],
    pub alignment_lev: &'a [AlgnSoft],
}

pub fn extractor_recipes() -> Vec<(
    Vec<Vec<f32>>,
    AlgnMergeAction,
    &'static dyn Fn(&[f32], &AlignmentPackage) -> Vec<AlgnHard>,
)> {
    vec![
        //(pack(&noparam()), optimizer::AlgnMergeAction::INTERSECT, &|p: &[f32]| align_hard::a1_argmax(&alignment_probs),
        (
            pack(&linspace(0.0, 0.05, 2)),
            AlgnMergeAction::INTERSECT,
            &|p: &[f32], package: &AlignmentPackage| {
                align_hard::a2_threshold(package.alignment_fwd, p[0])
            },
        ),
        (
            pack(&linspace(0.0, 0.0, 1)),
            AlgnMergeAction::INTERSECT,
            &|p: &[f32], package: &AlignmentPackage| {
                align_hard::a3_threshold_dynamic(package.alignment_fwd, p[0])
            },
        ),
        (
            pack(&linspace(0.95, 1.0, 4)),
            AlgnMergeAction::INTERSECT,
            &|p: &[f32], package: &AlignmentPackage| {
                align_hard::a4_threshold_dynamic(package.alignment_fwd, p[0])
            },
        ),
        (
            pack(&linspace(0.4, 0.8, 5)),
            AlgnMergeAction::INTERSECT,
            &|p: &[f32], package: &AlignmentPackage| {
                align_hard::a4_threshold_dynamic(package.alignment_diag, p[0])
            },
        ),
        (
            cartesian_product(vec![linspace(0.0, 0.2, 3), linspace(0.1, 0.3, 8)]),
            AlgnMergeAction::INTERSECT,
            &|p: &[f32], package: &AlignmentPackage| {
                align_hard::a2_threshold(&align_soft::misc::blur(package.alignment_fwd, p[0]), p[1])
            },
        ),
        (
            pack(&linspace(0.95, 1.0, 4)),
            AlgnMergeAction::INTERSECT,
            &|p: &[f32], package: &AlignmentPackage| {
                alignment_reverse(&align_hard::a4_threshold_dynamic(
                    package.alignment_rev,
                    p[0],
                ))
            },
        ),
        (
            pack(&linspace(0.7, 1.0, 4)),
            AlgnMergeAction::JOIN,
            &|p: &[f32], package: &AlignmentPackage| {
                align_hard::a2_threshold(package.alignment_lev, p[0])
            },
        ),
    ]
}
