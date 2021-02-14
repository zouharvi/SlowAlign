encs_search:
	cargo run --release --bin slow_align -- \
	--file1 data/data_encs.en --file2 data/data_encs.cs \
	--gold data/data_encs.algn --method search --gold-substract-one \
	--lowercase \
	1> /dev/null

encs_static:
	cargo run --release --bin slow_align -- \
	--file1 data/data_encs.en --file2 data/data_encs.cs \
	--gold data/data_encs.algn --method static --gold-substract-one \
	--lowercase \
	1> /dev/null

encs_save_dic:
	cargo run --release --bin slow_align_word_probs -- \
	data/data_encs.en data/data_encs.cs data/data_encs.dic \
	--threshold 0.1

encs_dic:
	cargo run --release --bin slow_align -- \
	--sent1 "The right focuses on efficiency ." --sent2 "Pravice se zaměřuje na efektivitu ." \
	--dic data/data_encs.dic --method dic \
	--params "[0.0],[0.0],[1.0],[0.8],[0.0,0.1],[0.95],[0.8]" \
	--lowercase