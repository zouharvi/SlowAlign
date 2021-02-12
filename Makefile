encs_search:
	cargo run --release --bin slow_align -- --file1 data/data_encs.en --file2 data/data_encs.cs --gold data/data_encs.algn --method search --gold-substract-one

encs_word_probs:
	cargo run --release --bin slow_align_word_probs -- data/data_encs.en data/data_encs.cs data/data_encs.dic --threshold 0.1

encs_dic:
	cargo run --release --bin slow_align -- --sent1 "The right focuses on efficiency ." --sent2 "Pravice se zaměřuje na efektivitu ." --dic data/data_encs.dic --method dic