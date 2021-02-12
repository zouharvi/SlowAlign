encs_search:
	cargo run --release --bin slow_align -- data/data_encs.en data/data_encs.cs --gold data/data_encs.algn --method search --gold-substract-one

encs_train:
	cargo run --release --bin slow_align_trainer -- data/data_encs.en data/data_encs.cs data/data_encs.dic --threshold 0.3