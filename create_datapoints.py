#!/usr/bin/env python
import os
import re
import sys
import glob
import time
import json
import codecs
import requests
from collections import OrderedDict
from pprint import pprint

# TODO: too many things from old python2 days, needs update

NONW = [
    u"to",
    u"in",
    u"the",
    u"am",
    u"or",
    u"and",
    u"a",
    u"of",
    u"at",
    u"on",
    u"is",
    u"that",
    u"this",
    u"into",
    u"was",
    u"were",
    u"",
]


def parse_file(path):
    """
    Parses the given path and returns a set of words for the post.
    """
    result = OrderedDict()
    # result = []
    date = ""
    with codecs.open(path, encoding="utf-8") as fobj:
        for i, line in enumerate(fobj):
            if line.startswith(".. tags:"):
                words = line[8:].strip()
                words = words.split(",")
                words = [word.strip() for word in words]
                for word in words:
                    result[word.lower()] = True
                # result.extend([word.strip() for word in words])
            if line.startswith(".. date:"):
                date = line.split(" ")[2].strip()
            if i < 7:
                continue

            line = remove_tags(line)
            words = line.split()
            for word in words:
                word = word.strip(' <>.,/?";:][{}\|~`!@#$%^&*()_+=-')
                if word in NONW:
                    continue
                result[word.lower()] = True

    return result, date


def remove_tags(line):
    cleanit = re.compile("<.*?>")
    return re.sub(cleanit, "", line)


def main():
    result = set()
    if len(sys.argv) != 2:
        print("Please provide the path to the directory of the source blog posts.")
        sys.exit(1)
    files = glob.glob(os.path.join(sys.argv[1], "*.md"))
    final_result = []
    for name in files:
        result, date = parse_file(name)
        # print result
        # continue
        final = []
        words = list(result.keys())
        for i, word in enumerate(words):
            if i > 0:
                final.append(u"%s %s" % (words[i - 1], words[i]))
            if i > 1:
                final.append(u"%s %s %s" % (words[i - 2], words[i - 1], words[i]))
            final.append(word)

        link = "https://kushaldas.in/posts/%s.html" % os.path.basename(name)[:-3]
        # This is dictionary (important for importing the data)
        data = {"words": final, "link": link, "date": date}
        final_result.append(data)
    with open("db.json", "w") as fobj:
        json.dump(final_result, fobj)


if __name__ == "__main__":
    main()
