# Exercises: Functional and Reactive Domain Modeling

A collection of exercises written in Rust to explore
[Functional and Reactive Domain Modeling](https://www.manning.com/books/functional-and-reactive-domain-modeling).

Functional and Reactive Domain Modeling teaches you how to think of the domain
model in terms of pure functions and how to compose them to build larger
abstractions.

* [Github](https://github.com/debasishg/frdomain)

## Domain Elements

### Entity

* has an identity
* passes through multiple states in the life-cycle
* usually has a definite life-cycle in the business

### Value Object

* semantically immutable
* can be freely shared across entities

### Services

* more macro-level abstraction than entity or value object
* involves multiple entities and value objects
* usually models a use case of the business
