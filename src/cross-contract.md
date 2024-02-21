# Cross contract communication

We already covered creating a single isolating contract. However, SOLID principles tell us that
entities should be as small as reasonably possible - such as they have a
[single responsibility](https://en.wikipedia.org/wiki/Single-responsibility_principle). Entities
we are focusing on now are smart contracts, and we want to make sure that every smart contract has
a sole responsibility it takes care of.

But we also want to build complex systems using smart contracts. To do so, we need to be able to
communicate between them. We already talked about what such communication looks like using an actor
model. Now it's time to use this knowledge in practice.

In this chapter, we will improve the previously created administration group model to solve the problem
I brought - the possibility of adding own multiple addresses by a single admin to take bigger donation parts.

We would also give admins some work to do besides being admins.
