#!/usr/bin/env python

import ja_sentence as ja
from gensim.models.doc2vec import Doc2Vec
from gensim.models.doc2vec import TaggedDocument
import sys

sentences = list(ja.convert_to_ja_sentences_from_files(sys.argv[1:]))

h = {}
tagged_docs = []
for i, s in enumerate(sentences):
  h[i] = s
  tagged_docs.append(TaggedDocument(words=s, tags=[i]))

model = Doc2Vec(documents=tagged_docs, vector_size=200, window=5, min_count=3, epochs=30)

def to_str(a):
  return "".join(a)

vocab = model.wv.vocab
while True:
  text = input('>> ')

  v = model.infer_vector(ja.convert_to_ja_sentences(text)[0])
  for pair in model.docvecs.most_similar(positive=[v]):
    j = pair[0]
    distance = pair[1]
    print(distance, to_str(h[j]))
