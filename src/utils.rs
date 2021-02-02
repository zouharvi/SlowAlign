pub mod reader;
pub mod evaluator;
pub mod cli;

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

pub fn linspace(start: f32, end: f32, steps: usize) -> Vec<f32> {
    (0..steps)
        .map(|step| {
            (end - start) * (step as f32) / {
                if steps <= 1 {
                    panic!("Number of steps in gridsearch has to be at least 2")
                }
                steps as f32 - 1.0
            } + start
        })
        .collect::<Vec<f32>>()
}

pub fn noparam() -> Vec<f32> {
    vec![0.0]
}