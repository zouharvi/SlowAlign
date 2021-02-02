pub fn a1_argmax(alignment_probs: &Vec<Vec<Vec<f32>>>) -> Vec<Vec<(usize, usize)>> {
    alignment_probs
        .iter()
        .map(|sent_prob| {
            sent_prob
                .iter()
                .enumerate()
                .map(|(pos1, tgt_probs)| {
                    (
                        pos1,
                        tgt_probs
                            .iter()
                            .enumerate()
                            .max_by(|(_, value0), (_, value1)| value0.partial_cmp(value1).unwrap())
                            .map(|(idx, _)| idx)
                            .unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}
