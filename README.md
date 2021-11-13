# XohImNooB Twitter Bot

He finds words in base64 encodings of SHA256 hashes. And tweet them.

Follow me! [twitter.com/XohImNoob](https://twitter.com/XohImNooB/)

# Huh?

So I teach a course on engineering secure software, and I was teaching about hashing, salt, and [rainbow tables](https://en.wikipedia.org/wiki/Rainbow_table). As an example I did the standard hash of a base64 encoding of sha256 of the world's worst password.... `password`.

And then a student started chuckling.

We all realized as a class that the hash was `XohImNooBHFR0OVvjcYpJ3NgPQ1qq73WKhHvch0VQtg=`, as in... "Oh I'm Noob". There's no way that's a coincidence, right?!? I'd like to think that there's some mathematician out there proud of themselves for getting an industry-standard hash function that generates a silly easter egg like that. Like... right?

So that leads us on the quixotic journey of seeing what else we can find in base64 encodings of sha256 hashes.


# Corpus?

I started from the [XKCD's Thing Explainer dictionary](https://xkcd.com/1133/), from the [simplewriter tool](https://xkcd.com/simplewriter/).

Removed one-letter words and added some others.
