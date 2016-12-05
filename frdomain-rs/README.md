# Exercises: Functional and Reactive Domain Modeling

A collection of exercises written in Rust to explore
[Functional and Reactive Domain Modeling](https://www.manning.com/books/functional-and-reactive-domain-modeling).

Functional and Reactive Domain Modeling teaches you how to think of the domain
model in terms of pure functions and how to compose them to build larger
abstractions.

* [Github](https://github.com/debasishg/frdomain)

## Ch 1-2. Domain Elements

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

## Ch 3. Designing Functional Domain Models

### The algebra of API Design

* Algebraic API design
* How it differs from OO design

In functional programming start with the _operations_ that correspond to the
basic domain behaviors and group related ones into _modules_ that form larger
use cases. Each behavior is modeled using functions that operate on types; the
types represent the data, class, or object of the domain.

### Defining the algebra of a domain service

* Designing a specific algebra
* Composing algebra
* Laws of algebra

### Lifecycle patterns of domain objects

* Factories
* Aggregates
* Repositories
* Functional Programming abstractions (lenses, monads)
* Dependency injection in Functional Programming
