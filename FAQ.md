
## How does aHash prevent DOS attacks

AHash is designed to [prevent an adversary that does not know the key from being able to create hash collisions or partial collisions.](https://github.com/tkaitchuck/aHash/wiki/How-aHash-is-resists-DOS-attacks)

If you are a cryptographer and would like to help review aHash's algorithm, please post a comment [here](https://github.com/tkaitchuck/aHash/issues/11).

In short, this is achieved by ensuring that:

* aHash is designed to [resist differential crypto analysis](https://github.com/tkaitchuck/aHash/wiki/How-aHash-is-resists-DOS-attacks#differential-analysis). Meaning it should not be possible to devise a scheme to "cancel" out a modification of the internal state from a block of input via some corresponding change in a subsequent block of input.
  * This is achieved by not performing any "premixing" - This reversible mixing gave previous hashes such as murmurhash confidence in their quality, but could be undone by a deliberate attack.
  * Before it is used each chunk of input is "masked" such as by xoring it with an unpredictable value.
* aHash obeys the '[strict avalanche criterion](https://en.wikipedia.org/wiki/Avalanche_effect#Strict_avalanche_criterion)':
Each bit of input has the potential to flip every bit of the output.
* Similarly, each bit in the key can affect every bit in the output.
* Input bits never effect just one, or a very few, bits in intermediate state. This is specifically designed to prevent the sort of 
[differential attacks launched by the sipHash authors](https://emboss.github.io/blog/2012/12/14/breaking-murmur-hash-flooding-dos-reloaded/) which cancel previous inputs.
* The `finish` call at the end of the hash is designed to not expose individual bits of the internal state. 
  * For example in the main algorithm 256bits of state and 256bits of keys are reduced to 64 total bits using 3 rounds of AES encryption. 
Reversing this is more than non-trivial. Most of the information is by definition gone, and any given bit of the internal state is fully diffused across the output.
* In both aHash and its fallback the internal state is divided into two halves which are updated by two unrelated techniques using the same input. - This means that if there is a way to attack one of them it likely won't be able to attack both of them at the same time.
* It is deliberately difficult to 'chain' collisions. (This has been the major technique used to weaponize attacks on other hash functions)

More details are available on [the wiki](https://github.com/tkaitchuck/aHash/wiki/How-aHash-is-resists-DOS-attacks).

## Why not use a cryptographic hash in a hashmap.

Cryptographic hashes are designed to make is nearly impossible to find two items that collide when the attacker has full control
over the input. This has several implications:

* They are very difficult to construct, and have to go to a lot of effort to ensure that collisions are not possible.
* They have no notion of a 'key'. Rather, they are fully deterministic and provide exactly one hash for a given input.

For a HashMap the requirements are different.

* Speed is very important, especially for short inputs. Often the key for a HashMap is a single `u32` or similar, and to be effective
the bucket that it should be hashed to needs to be computed in just a few CPU cycles.
* A hashmap does not need to provide a hard and fast guarantee that no two inputs will ever collide. Hence, hashCodes are not 256bits 
but are just 64 or 32 bits in length. Often the first thing done with the hashcode is to truncate it further to compute which among a few buckets should be used for a key. 
  * Here collisions are expected, and a cheap to deal with provided there is no systematic way to generated huge numbers of values that all
go to the same bucket.
  * This also means that unlike a cryptographic hash partial collisions matter. It doesn't do a hashmap any good to produce a unique 256bit hash if
the lower 12 bits are all the same. This means that even a provably irreversible hash would not offer protection from a DOS attack in a hashmap
because an attacker can easily just brute force the bottom N bits.

From a cryptography point of view, a hashmap needs something closer to a block cypher.
Where the input can be quickly mixed in a way that cannot be reversed without knowing a key.

## Why isn't aHash cryptographically secure

It is not designed to be. 
Attempting to use aHash as a secure hash will likely fail to hold up for several reasons:

1. aHash relies on random keys which are assumed to not be observable by an attacker. For a cryptographic hash all inputs can be seen and controlled by the attacker.
2. aHash has not yet gone through peer review, which is a pre-requisite for security critical algorithms.
3. Because aHash uses reduced rounds of AES as opposed to the standard of 10. Things like the SQUARE attack apply to part of the internal state.