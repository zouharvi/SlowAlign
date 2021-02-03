use crate::evaluator::AlgnSoft;
pub mod ibm1;
pub mod misc;

pub fn merge_sum(alignment1: &[AlgnSoft], alignment2: &[AlgnSoft], weight1: f32) {
    let weight2 = 1.0 - weight1;
    assert_eq!(alignment1.len(), alignment2.len());
    let mut scores = alignment1
        .iter()
        .zip(alignment2)
        .map(|(sent_a1, sent_a2)| {
            vec![
                vec![
                    0.0;
                    {
                        assert_eq!(sent_a1[0].len(), sent_a2[0].len());
                        sent_a2[0].len()
                    }
                ];
                {
                    assert_eq!(sent_a1.len(), sent_a2.len());
                    sent_a1.len()
                }
            ]
        })
        .collect::<Vec<AlgnSoft>>();
    for (sent_a1, sent_a2) in alignment1.iter().zip(alignment2) {
        // TODO
    }
}
