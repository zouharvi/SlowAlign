#!/bin/env bash

# generate two files (data_csen.algn and data_csen.sent) out of CzEnAli_1.0 dataset folder

cat CzEnAli_1.0/data/**/*.wa | grep -e '<english>'  | sed -e 's/\s*<[^>]*>//g' > /tmp/text.en
cat CzEnAli_1.0/data/**/*.wa | grep -e '<czech>'    | sed -e 's/\s*<[^>]*>//g' > /tmp/text.cs
cat CzEnAli_1.0/data/**/*.wa | grep -e '<sure>'     | sed -e 's/\s*<[^>]*>//g' > /tmp/text.sure
cat CzEnAli_1.0/data/**/*.wa | grep -e '<possible>' | sed -e 's/\s*<[^>]*>//g;s/-/?/g' > /tmp/text.poss

get_seeded_random()
{
  seed="$1"
  openssl enc -aes-256-ctr -pass pass:"$seed" -nosalt \
    </dev/zero 2>/dev/null
}

paste -d ' ' /tmp/text.sure /tmp/text.poss > /tmp/text.algn

shuf --random-source=<(get_seeded_random 64) /tmp/text.en > data_csen.en &
shuf --random-source=<(get_seeded_random 64) /tmp/text.cs > data_csen.cs &
shuf --random-source=<(get_seeded_random 64) /tmp/text.algn > data_csen.algn &
wait

echo -n "Sentences: "
cat /tmp/text.en | wc -l

echo -n "Tokens CS: "
cat /tmp/text.cs | tr " " "\n" | wc -l

echo -n "Tokens EN: "
cat /tmp/text.en | tr " " "\n" | wc -l