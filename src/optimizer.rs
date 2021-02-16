use crate::evaluator::{alignment_error_rate, alignment_reverse};
use crate::evaluator::{AlgnGold, AlgnHard, AlgnSoft};
use crate::linspace;
use crate::pack;
use crate::utils::cartesian_product;
use crate::{align_hard, align_soft};

pub type Extractor<T> = &'static dyn Fn(&[T], &AlignmentPackage) -> Vec<AlgnHard>;

#[derive(Copy, Clone)]
pub enum AlgnMergeAction {
    JOIN,
    INTERSECT,
}

/**
 * Intersect hard alignments.
 **/
pub fn intersect_algn(running: Option<Vec<AlgnHard>>, new: Vec<AlgnHard>) -> Option<Vec<AlgnHard>> {
    if let Some(running) = running {
        Some(
            running
                .iter()
                .zip(new)
                .map(|(old, new)| old.intersection(&new).copied().collect::<AlgnHard>())
                .collect::<Vec<AlgnHard>>(),
        )
    } else {
        Some(new)
    }
}

/**
 * Join hard alignments
 **/
pub fn join_algn(running: Option<Vec<AlgnHard>>, new: Vec<AlgnHard>) -> Option<Vec<AlgnHard>> {
    if let Some(running) = running {
        Some(
            running
                .iter()
                .zip(new)
                .map(|(old, new)| old.union(&new).copied().collect::<AlgnHard>())
                .collect::<Vec<AlgnHard>>(),
        )
    } else {
        Some(new)
    }
}

/**
 * Compute hard alignment given the necessary precomputed package, the extractor list and the parameters
 **/
pub fn params_to_alignment<T>(
    params: &[Vec<T>],
    package: &AlignmentPackage,
    extractors: &[(AlgnMergeAction, Extractor<T>)],
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

/**
 * Perform (slow) grid search.
 **/
pub fn gridsearch<T>(
    package: &AlignmentPackage,
    extractor_params: &[Vec<Vec<T>>],
    extractors: &[(AlgnMergeAction, Extractor<T>)],
    gold_algn: &[AlgnGold],
) -> (Vec<AlgnHard>, Vec<Vec<T>>, f32)
where
    T: Clone,
    T: std::fmt::Debug,
    T: std::marker::Sized,
{
    // create linspace ranges
    let grid = cartesian_product(extractor_params);

    if grid.is_empty() {
        panic!("Empty list of parameters to optimize")
    }

    let mut min_aer = f32::INFINITY;
    let mut best_params: Option<Vec<Vec<T>>> = None;
    let mut best_algn: Option<Vec<AlgnHard>> = None;

    for params in grid {
        eprintln!("Trying {:?} ", params);
        assert_eq!(params.len(), extractors.len());

        let algn = params_to_alignment(&params, package, &extractors);
        let aer = alignment_error_rate(&algn, gold_algn);
        eprintln!("AER {:.4}, best {:.4}\n", aer, min_aer);
        if aer < min_aer {
            best_params = Some(params);
            min_aer = aer;
            best_algn = Some(algn);
        }
    }

    eprintln!("Best AER: {}", min_aer);
    eprintln!("Best Params: {:?}\n", best_params.clone().unwrap());

    (best_algn.unwrap(), best_params.unwrap(), min_aer)
}

/**
 * Struct to hold pre-computed soft alignments used for extractors.
 **/
pub struct AlignmentPackage<'a> {
    pub alignment_fwd: &'a [AlgnSoft],
    pub alignment_rev: &'a [AlgnSoft],
    pub alignment_diag: &'a [AlgnSoft],
    pub alignment_lev: &'a [AlgnSoft],
}

/**
 * Set of extractor recipes used in the gridsearch and the inference.
 **/
pub const EXTRACTOR_RECIPES: &[(AlgnMergeAction, Extractor<f32>)] = &[
    (
        AlgnMergeAction::INTERSECT,
        &|p: &[f32], package: &AlignmentPackage| {
            align_hard::a4_threshold_dynamic(package.alignment_fwd, p[0])
        },
    ),
    (
        AlgnMergeAction::INTERSECT,
        &|p: &[f32], package: &AlignmentPackage| {
            alignment_reverse(&align_hard::a3_threshold_dynamic(
                package.alignment_rev,
                p[0],
            ))
        },
    ),
    (
        AlgnMergeAction::INTERSECT,
        &|p: &[f32], package: &AlignmentPackage| {
            align_hard::a2_threshold(package.alignment_diag, p[0])
        },
    ),
    (
        AlgnMergeAction::INTERSECT,
        &|p: &[f32], package: &AlignmentPackage| {
            align_hard::a2_threshold(&align_soft::misc::blur(package.alignment_fwd, p[1]), p[0])
        },
    ),
    (
        AlgnMergeAction::JOIN,
        &|p: &[f32], package: &AlignmentPackage| {
            align_hard::a2_threshold(package.alignment_lev, p[0])
        },
    ),
    (
        AlgnMergeAction::INTERSECT,
        &|p: &[f32], package: &AlignmentPackage| {
            align_hard::a2_threshold(package.alignment_fwd, p[0])
        },
    ),
];

/**
 * Parameter space for the extractor recipes.
 **/
pub fn extractor_recipes_params() -> Vec<Vec<Vec<f32>>> {
    vec![
        pack(&linspace(0.95, 1.0, 4)),
        pack(&linspace(0.90, 1.0, 6)),
        pack(&linspace(0.1, 1.0, 10)),
        cartesian_product(&[linspace(0.1, 0.3, 8), linspace(0.0, 0.005, 4)]),
        pack(&linspace(0.7, 1.0, 4)),
        pack(&linspace(0.0, 0.005, 4)),
    ]
}
