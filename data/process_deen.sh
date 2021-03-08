#!/bin/env bash

cp data_deen_a /tmp/text_a
cp data_deen_s /tmp/text_s
cp data_deen_t /tmp/text_t

get_seeded_random()
{
  seed="$1"
  openssl enc -aes-256-ctr -pass pass:"$seed" -nosalt \
    </dev/zero 2>/dev/null
}

shuf --random-source=<(get_seeded_random 64) /tmp/text_s > data_deen_s &
shuf --random-source=<(get_seeded_random 64) /tmp/text_t > data_deen_t &
shuf --random-source=<(get_seeded_random 64) /tmp/text_a > data_deen_a &
wait

echo -n "Sentences: "
cat /tmp/text_s | wc -l

echo -n "Tokens CS: "
cat /tmp/text_t | tr " " "\n" | wc -l

echo -n "Tokens EN: "
cat /tmp/text_s | tr " " "\n" | wc -l