#!/bin/env bash

# generate two files (data_encs_a and data_encs.sent) out of CzEnAli_1.0 dataset folder

cat CzEnAli_1.0/data/**/*.wa | grep -e '<english>'  | sed -e 's/\s*<[^>]*>//g' > /tmp/text_s
cat CzEnAli_1.0/data/**/*.wa | grep -e '<czech>'    | sed -e 's/\s*<[^>]*>//g' > /tmp/text_t
cat CzEnAli_1.0/data/**/*.wa | grep -e '<sure>'     | sed -e 's/\s*<[^>]*>//g' > /tmp/text.sure
cat CzEnAli_1.0/data/**/*.wa | grep -e '<possible>' | sed -e 's/\s*<[^>]*>//g;s/-/?/g' > /tmp/text.poss

get_seeded_random()
{
  seed="$1"
  openssl enc -aes-256-ctr -pass pass:"$seed" -nosalt \
    </dev/zero 2>/dev/null
}

paste -d ' ' /tmp/text.sure /tmp/text.poss > /tmp/text_a

shuf --random-source=<(get_seeded_random 64) /tmp/text_s > data_encs_s &
shuf --random-source=<(get_seeded_random 64) /tmp/text_t > data_encs_t &
shuf --random-source=<(get_seeded_random 64) /tmp/text_a > data_encs_a &
wait

echo -n "Sentences: "
cat /tmp/text_s | wc -l

echo -n "Tokens CS: "
cat /tmp/text_t | tr " " "\n" | wc -l

echo -n "Tokens EN: "
cat /tmp/text_s | tr " " "\n" | wc -l