#!/usr/bin/env python

import MeCab
import re

def convert_to_ja_sentences(text):
  mecab = MeCab.Tagger('-Owakati')
  sentences = []
  line = []
  for word in re.split(r'\s+', mecab.parse(text)):
    if not word:
      continue
    if re.search(r'。|\.|．|「|」', word):
      if line:
        sentences.append(line)
      line = []
    else:
      line.append(word)
  if line:
    sentences.append(line)
  return sentences

def convert_to_ja_sentences_from_files(paths):
  for path in paths:
    with open(path) as f:
      for s in convert_to_ja_sentences(f.read()):
        yield s
