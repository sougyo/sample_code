#!/usr/bin/env python

import ja_sentence as ja
from gensim.models import word2vec
import sys

sentences = list(ja.convert_to_ja_sentences_from_files(sys.argv[1:]))

model = word2vec.Word2Vec(sentences, size=100, min_count=3, window=3, iter=30)

#def out(wv, path):
#  wv.save(path)
#  for k in wv.vocab:
#    print(k + " " + " ".join([str(v) for v in wv.word_vec(k)]))

vocab = model.wv.vocab
while True:
  text = input('>> ')

  if not (text in vocab):
    continue

  result = model.most_similar(positive=[text])

  for pair in result:
    word = pair[0]
    distance = pair[1]
    print(word, distance)

