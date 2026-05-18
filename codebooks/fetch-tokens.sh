# Fetches word lists from david47k top_english_wordslists
curl -o english-full.txt https://raw.githubusercontent.com/david47k/top-english-wordlists/master/top_english_words_lower_100000.txt
head -n 60000 english-full.txt > english-60k.txt
wc -l english-60k.txt