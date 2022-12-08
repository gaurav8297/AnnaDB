# [WIP] AnnaDB - Rust Implementation

Rewrite of the research project, Anna, in Rust with better fault tolerance for making it production-ready.  It uses the concept of coordination-free consistency described in the CALM theorem.

## Status

- This is a very dirty wip implementation of [AnnaDb](https://github.com/hydro-project/anna) in Rust to understand the concepts better.
- For now, I've left this in a middle to focus on implementing a good fault tolerant membership protocol. I'll be going to use that for AnnaDB to hide the complexity of networking code thus making the implementation simpler.
    - https://github.com/gaurav8297/omega 

## Why Rust?

- Memory safety
- No runtime
- No gc so memory can be optimized
- Best in class compiler which makes it easy to maintain and few errors
- Cons: Slow for development because of same compiler

## Research

- http://db.cs.berkeley.edu/jmh/papers/anna_ieee18.pdf [must read]
- http://bloom-lang.net/calm/ [must read]
- http://www.vikrams.io/papers/anna-vldb19.pdf [must read]
- http://www.jmfaleiro.com/pubs/latch-free-cidr2017.pdf [must read]
- https://arxiv.org/pdf/1901.01930.pdf
- https://www.cs.cornell.edu/projects/ladis2009/papers/lakshman-ladis2009.pdf
- https://www.scylladb.com/
- https://github.com/papers-we-love/papers-we-love/blob/master/datastores/dynamo-amazons-highly-available-key-value-store.pdf
