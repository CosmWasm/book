# Cross contract communication

We have covered creating a single isolated contract. However, SOLID principles tell us that
entities should be as small as reasonably possible so that they have a
[single responsibility](https://en.wikipedia.org/wiki/Single-responsibility_principle). The entities
we are focusing on now are smart contracts, and we want to ensure that every smart contract has
the sole responsibility for whatever specific task it takes care of.

However, we also want to build complex systems using smart contracts. To do so, we need to be able to
communicate between them and we discussed how such communication looks using the Actor
Model. Now it's time to use this knowledge in practice.

In this chapter we will improve the administration group model we created before to solve the problem
we presented earlier - the possibility of an admin adding multiple addresses in order to take a greater proportion of the donations.

We shall also give admins some work to do besides just being admins.
