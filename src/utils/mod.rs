pub mod cli;
pub mod evaluator;
pub mod reader;
pub mod writer;

/**
 * Generate a cartesian product of multiple vectors.
 **/
pub fn cartesian_product<T>(lists: &[Vec<T>]) -> Vec<Vec<T>>
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

/**
 * Transpose a matrix (vector of vectors).
 **/
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

/**
 * Wrap every element in a singleton vector.
 **/
pub fn pack<T>(range: &[T]) -> Vec<Vec<T>>
where
    T: Copy,
{
    range.iter().map(|x| vec![*x]).collect()
}

/**
 * Generate a linear space given interval boundaries and number of steps.
 **/
pub fn linspace(start: f32, end: f32, steps: usize) -> Vec<f32> {
    if steps <= 1 {
        #[allow(clippy::float_cmp)]
        if start != end {
            panic!("In case of number of steps less or equal to 1, the start and end has to match.")
        }
        return vec![start];
    }
    (0..steps)
        .map(|step| (end - start) * (step as f32) / { steps as f32 - 1.0 } + start)
        .collect::<Vec<f32>>()
}

/**
 * Compute levenstein distance of two words.
 **/
pub fn levenstein_distance(word1: &str, word2: &str) -> usize {
    let w1 = word1.chars().collect::<Vec<_>>();
    let w2 = word2.chars().collect::<Vec<_>>();
    let mut matrix = vec![vec![0; w1.len() + 1]; w2.len() + 1];

    for i in 0..w1.len() {
        matrix[0][i + 1] = i + 1;
    }
    for j in 0..w2.len() {
        matrix[j + 1][0] = j + 1;
    }

    for j in 0..w2.len() {
        for i in 0..w1.len() {
            matrix[j + 1][i + 1] = if w1[i] == w2[j] {
                matrix[j][i]
            } else {
                1 + std::cmp::min(
                    std::cmp::min(matrix[j + 1][i], matrix[j][i + 1]),
                    matrix[j][i],
                )
            };
        }
    }
    matrix[w2.len()][w1.len()]
}

/**
 * Computes normalized [0..1] score based on word similarity using the levenstein distance.
 * 0 - words dissimiliar, 1 - words identical
 */
pub fn levenstein_score(word1: &str, word2: &str) -> f32 {
    1.0 - (levenstein_distance(word1, word2) as f32) / ((word1.len() + word2.len()) as f32)
}

/**
 * Compute the argmax of a vector.
 **/
pub fn argmax(probs: &[f32]) -> usize {
    probs
        .iter()
        .enumerate()
        .max_by(|(_, value0), (_, value1)| value0.partial_cmp(value1).unwrap())
        .map(|(idx, _)| idx)
        .unwrap()
}
