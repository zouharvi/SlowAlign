encs_search:
	cargo run --release --bin slow_align -- \
	--file1 data/data_encs_s --file2 data/data_encs_t \
	--gold data/data_encs_a --method search --gold-index-one \
	--lowercase --dev-count 20 --ibm-steps 10 \
	1> /dev/null

fren_search:
	cargo run --release --bin slow_align -- \
	--file1 data/data_fren_s --file2 data/data_fren_t \
	--gold data/data_fren_a --method search \
	--lowercase --dev-count 37 --test-offset 0 --ibm-steps 5 \
	1> /dev/null

fren_static:
	cargo run --release --bin slow_align -- \
	--file1 data/data_fren_s --file2 data/data_fren_t \
	--gold data/data_fren_a --method static \
	--lowercase --dev-count 37 --test-offset 0 \
	1> /dev/null

encs_static:
	cargo run --release --bin slow_align -- \
	--file1 data/data_encs.en --file2 data/data_encs_t \
	--gold data/data_encs_a --method static --gold-index-one \
	--lowercase \
	1> /dev/null

encs_save_dic:
	cargo run --release --bin slow_align_word_probs -- \
	data/data_encs.en data/data_encs_t data/data_encs.dic \
	--threshold 0.1

encs_dic:
	cargo run --release --bin slow_align -- \
	--sent1 "The right focuses on efficiency ." --sent2 "Pravice se zaměřuje na efektivitu ." \
	--dic data/data_encs.dic --method dic \
	--params "[0.0],[0.0],[1.0],[0.8],[0.0,0.1],[0.95],[0.8]" \
	--lowercase