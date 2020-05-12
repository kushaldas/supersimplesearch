## super simple search


This is the Rust implementation of my <https://search.kushaldas.in>.

There is a Python script called `create_datapoints.py` which takes the static blog post
directory as input, and creates `db.json` file out of it. This code is old (uses Python2),
and still needs clean up to make it for general use.

Which is a list of entries like the following dictionary.

```
{
    "link": "https://kushaldas.in/whatever.html",
     "date": "2007-12-18T11:57:13+05:30",
     "words":  [
            "news",
            "news python",
            "python",
            "python description",
            "news python description"]
}
```



## License: GPLv3+

