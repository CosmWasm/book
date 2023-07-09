# Idea behind the Actor Model

The Actor Model is a solution to the problem of communication between smart
contracts. Let's take a look at the reasons this particular solution was chosen for CosmWasm and some of the consequences.

## The problem

Smart contracts can be conceived of as sandboxed microservices. According to
[SOLID](https://en.wikipedia.org/wiki/SOLID) principles, it is valuable to
split responsibilities between entities. In order to split the work between
contracts it is necessary for them to be able to communicate with each other.  So if one
contract is responsible for managing group membership, for example, it should be possible to call
its functionality from another contract.

The traditional way to solve this problem in software engineering is to model
services as functions that can be called using some [RPC](https://en.wikipedia.org/wiki/Remote_procedure_call) mechanism, and return
the results as responses. Even though this approach looks nice, it can create all sorts
of problems especially with maintaining a consistent shared state .

Another approach, which is far more popular in business-level modeling, is to
instead treat entities as actors that can perform tasks that are not interrupted by calls to other contracts. Any calls to other contracts can
only be made after the entire execution is completed. When a  "sub-call" is
finished, it calls the original (calling) contract back.

This solution may feel unnatural at first, and it certainly requires different kinds of design
solutions, but it turns out that it works pretty well for smart contract execution.
Below we will try to explain how to reason about it, and describe how it maps to contract
structure.

## The Actor

So what is an "actor"? An actor is a single instantiation of a contract, able to perform
one or more actions. When the actor finishes its job, it prepares a summary including a list of tasks carried out, as the final step in completing the scheduled task.

A (metaphorical) example of an actor is a cashier in a KFC restaurant. When you make an order, you are requesting an action from the cashier. From your perspective (that of the user of the system)
, you can think about this task as "sell and prepare my meal", but in fact
the action performed by the cashier is simply "Take payment and create order".
The first part of this operation is to create a bill and charge you for it, and
then the cashier requests that your sandwich and fries are prepared by another actor,
probably the chefs. When each of the chefs is done with their part of the meal, one checks
if all meals are ready. If so, the chef calls the last actor, the waiter, to deliver
the food to you. Upon receipt of your order, the task is
considered complete.

The workflow described above is of course simplified. In particular, in a
typical restaurant a waiter would probably observe the kitchen instead of being
triggered by a chef, something that is not possible in the Actor Model. Here, entities
of the system are passive and cannot observe the environment actively - they
only react to messages from other system participants. Also, in the case of restaurants such as KFC, the cashier would not schedule subtasks for particular chefs. Instead, they would leave tasks
open to be taken by the chefs whenever one of them is free to do so. In the Actor Model this is not the case, for the same reason as before -
chefs cannot actively listen to the environment. However, it would be possible
to create a contract to act as a chef's dispatcher that would collect all
orders from the cashier and then balance them across chefs for example.

## The Action

Actors are the model's entities and to properly communicate with them we need
some kind of protocol. Every actor may be capable of performing several actions. In the
KFC example above, the only action a cashier can do is "Take payment and
create order". On the other hand, our chefs are proficient
at performing both "Prepare fries" and "Prepare Sandwich" actions and many more.

So, when we want to do something in this system of actors, we schedule (request) some action to
be performed by the actor closest to us, very often with some additional parameters
(for example, so that we can decide if we want to exchange the fries with salad).

Naming an action after the exact thing which happens in the
contract can be misleading. Let's take a look at the KFC example once again. As 
mentioned, the action performed by a cashier is "Take payment and create
order". Note that for the customer who schedules this action, it
doesn't actually matter what the precise responsibility of the actor called is.  The customer simply schedules (requests) "Prepare Meal" with some description of what
exactly to prepare. So it can be clearer to define the action as the process performed by
the contract itself, plus all the sub-actions it schedules.

## Multi-stage Actions

Although the whole idea hopefully makes some sense now, there is admittedly a problem created by the
Actor Model: what if, in order to finalize the steps during performance of an action, the contract needs to be sure that some
sub-action that was scheduled is finished?

Imagine that in the previous KFC scenario, there is no dedicated waiter but instead it is actually the cashier who serves you the meal once the chefs have finished preparing it.

This kind of pattern is so important and common that in CosmWasm, we developed
a special way to handle it, which takes the form of a dedicated `Reply` action.

So when a cashier is scheduling actions for chefs, they assign some number to this
action (like an order id) and pass this on to the chefs. They also remember how many
actions they have scheduled for every order id. Now every time a chef completes an action they then call the cashier with a special `Reply` action including the order id. Upon receiving this, the cashier decreases the number of actions
left for this order.  Once this number reaches zero, the cashier will serve the meal.

Now you could say that the `Reply` action is not really necessary, since the chefs
could in any case just schedule any arbitrary action on the cashier, like `Serve`. So what is the point of this special `Reply` action? The answer is: for abstraction and reusability. The chef's
task is to prepare a meal and that is all. There is no reason for them to know
why they are even preparing fries, whether it is part of the bigger task (like an order
for a customer) or whether the cashier just happens to be hungry. It is possible that others apart from the cashier are able to call the chef to order preparation of food - perhaps any restaurant employee can do it! Therefore we need a way to be able to react
to an actor finishing their job in some universal way, enabling us to handle this situation
properly in any context.

It is worth noting also that the `Reply` action can contain some additional data. The id
assigned previously is the only required information in the `Reply` call, but
the actor can pass some additional data such as `events` emitted, for example, which are mostly
metadata (mainly for observation by non-blockchain applications), and any other
arbitrary data it wishes to pass.

## State

Up until this point, we have been considering actors as entities performing a
task such as preparing a meal. If we are considering computer programs, such a
task would be something like showing something on the screen or maybe printing something. This is
not the case with smart contracts. The only thing which can be affected by the
smart contract is its own internal state. The state is arbitrary data that is
kept by the contract. In the KFC example above, the cashier keeps in mind how many actions they scheduled for chefs that are not yet finished -
this number is part of the cashier's state.

Let's think about a more realistic smart contract than our restaurant example. Let's imagine we want to create
our own currency - perhaps we want to create a smart contract based market for
a game. We need some way to be able to at least transfer currency
between players. We can do that by creating a contract we could call
`MmoCurrency`, which will support the `Transfer` action to transfer money to
another player. So what would be the state of such a contract? It would be
just a table mapping player names to the amount of currency they own. The
contract we just invented exists in CosmWasm examples, and it is called the
[`cw20-base`
contract](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base)
(actually it's a bit more complicated, but this is its core idea).

And now the question arises - how helpful is it to be able to transfer currency if we
cannot check how much of it we own? It's a very good question, and the
answer is simple - the whole state of every contract in our system is
public. This is not universally true for every Actor Model, but this is how it works in
CosmWasm and in fact this is kind of forced on us by the nature of blockchain. Everything
happening in the blockchain has to be public, and if some information needs to be
hidden then it has to be stored indirectly.

There is one more very important thing to note about the state in CosmWasm, and that is that the
state is "transactional". Any updates to the state are not applied
immediately, but only when a whole action succeeds. This is a very important property, as
it guarantees that if something goes wrong in the running of a contract, it is always left
in some valid state. Let's consider our `MmoCurrency` case. Imagine that within
the `Transfer` action functionality we first increase the receiver's currency balance (by
updating the state), and only afterwards do we attempt to decrease the sender's balance. Before decreasing it, we of course need to check if the sender possesses enough funds to
perform the transaction. If we realize that there are not enough funds, we don't need
to do any rolling back by hand - we can just return a failure from the action
execution function, and the state would not be updated. So, within the running of the contract, when contract state is updated, it is actually just a local copy of this state being altered, but the
partial changes are never made visible to other contracts.

## Queries

There is one other building block in the CosmWasm approach to the Actor model which
we haven't yet covered. As mentioned, the whole state of every contract is public and
available for anyone to look at. The problem with this is that this way of looking at
state is not very convenient - it requires users of a contract to know its
internal structure, which seems to violate the SOLID principles (Liskov substitution
principle in particular). For example, if a contract is updated and its state
structure changes a bit, another contract looking at its state would just
no longer work. Also, it is often the case that the actual contract state is somewhat
simplified and the information that is relevant to the observer needs to be
calculated from the state.

This is where queries come into play. Queries are the type of messages sent to a
contract that do not require the performance of any actions and so do not update the state.  Instead, they return an answer immediately.

In our KFC example, an example of a query would be if the cashier goes to a chef and asks "Do we
still have pickles available for our cheeseburgers"? This can be done while
operating, and a response can be used within our operations. This is possible because queries can
never update their state, so they do not need to be handled in a transactional
manner.

Of course, the existence of queries doesn't mean that we cannot look at the
contract's state directly. The state is still public, and the technique of
looking at them directly is called `Raw Queries`. For clarity, non-raw queries
are sometimes referred to as `Smart Queries`.

## Wrapping everything together - transactional call flow

We touched on many things here, and it may be a little confusing.
Because of this, let's go through some more complicated calls to the
CosmWasm contract to better visualize what the "transactional state" means.

Let's imagine two contracts:

1. The `MmoCurrency` contract mentioned before, which can perform the
   `Transfer` action, allowing transfer of some `amount` of currency to some
   `receiver`.
2. The `WarriorNpc` contract, which has some amount of our currency, and
   is used by our MMO engine to pay the reward out for some quest
   a player could perform. It will be triggered by a `Payout` action, which can be
   called only by a specific client (which would be our game engine).

Now here is the interesting thing: this model forces us to make our MMO more
realistic in terms of the economy that we traditionally see since
`WarriorNpc` has a limited quantity of currency and cannot just create more out of
nothing. This is not always the case (the previously mentioned `cw20` has the
notion of "minting" for this case), but for the sake of simplicity let's assume this
is what we want.

To make the quest attractive for longer, we could make sure that the reward for it is 
always between `1 mmo` and `100 mmo`, but it will be ideally be `15%` of what
`WarriorNpc` owns. This means that the quest reward decreases for every subsequent
player, until eventually the `WarriorNpc` contract is broke, left with nothing, and so no longer able to make payouts to players.

So, what would the flow look like? We start with the game sending a `Payout` message
to the `WarriorNpc` contract, with info on who should receive the reward. The Warrior
keeps track of all the players who have fulfilled the quest, so as not to pay out the same
person twice, so its state will always contain such a list of players. The first thing the contract will do is check this list and, if the new player is already present in the list, the contract finishes
returning an error.

However, in most cases the player will not be on the list - so then
`WarriorNpc` will add the player to the list. Now the Warrior will finish its part
of the task, and schedule the `Transfer` action to be performed by the
`MmoCurrency` contract.

But here is the crucial part: since the `Transfer` action is actually part
of the larger `Payout` flow, it will not be executed on the original
blockchain state, but on the local copy of it.  So if the `MmoCurrency` should for any reason take a look
at `WarriorNpc`'s internal list, it will already appear updated including the new player.

Now `MmoCurrency` does its job, updating the state of Warrior and player
balances (note that here our Warrior is treated just as like any other player!). There are two possibilities:

1. There was an error - possibly the Warrior is out of cash and can no longer
   pay for the task. In this case, none of the changes (neither the update of the
   list of players, nor the balance changes) are applied to the
   original blockchain storage.  It's as if they never happened. In the
   database world, this is known as "rolling back the transaction".
2. The operation is successful - all changes on the state are now applied to the
   blockchain, and any further observation of `MmoCurrency` or `WarriorNpc` by
   the external world would see updated data.

There is still a small issue to consider. In this model, our list is not really a list of players who have fulfilled the quest (as we originally intended), but rather a list of players who have been paid
out.  This is because in the case of the transfer failing, the list is not updated. We can do better!

## Different ways of handling responses

Note that we didn't mention a `Reply` operation at all. So why was it not
called by `MmoCurrency` on `WarriorNpc`? The reason is that this operation is
optional. When scheduling sub-actions on another contract we may choose how 
`Reply` can be utilised, with one of the following options:

1. Never call `Reply`, action fails if sub-message fails
2. Call `Reply` on success
3. Call `Reply` on failure
4. Always call `Reply`

So if we do not explicitly request that `Reply` should called by the subsequent contract, it won't
 happen. In that case, whenever a sub-call fails then the whole transaction is rolled
back i.e. sub-message failure transitively causes original message failure. This may all seem a bit complicated right now, but we promise it gets easier with practice.

When handling the reply, it is important to remember that although changes have
not yet been applied to the blockchain itself (since the transaction may still fail), the
reply handler is already working on the local copy of the state containing all changes made
so far by the sub-message. In most cases this is a good thing, but it
has one tricky consequence.  If a contract calls itself recursively, it becomes
possible that subsequent calls overwrite things set up in the original message.
Though this rarely happens, we may need special treatment in some cases.  For now we
don't need to go too deeply into the details, but you should bear this in mind.

Now let's take a look at handling results with options `2` to `4`. Option `2` is quite interesting: even if the transaction performed by the sub-call succeeds, we may nevertheless take a look at the data it returned with `Reply` as well as its final state after finishing, and still decide that the action as a
whole is a failure.  In this case everything will be rolled back, including any
currency transfers performed by the external contract.

In our case, it is actually option `3` that is most useful. If the contract calls
`Reply` on failure, we can still decide to claim success for the action as a whole, and commit a transaction on
the state even though the sub call failed. This is what we want, since our internal list is supposed to keep a list of players who succeeded at the quest, whether or not the rewards were paid out. Even if we have no more currency to pay out, we still want to update this list.

The most common way to use the replies (option `2` in particular) is to
instantiate another contract, managed by the one called. The idea is that in
those use cases, the creator contract wants to keep the address of the created
contract in its state. To do so it has to create an `Instantiate` sub-message,
and subscribe for its success response, which contains the address of the freshly
created contract.

In the end, you can see that performing actions in CosmWasm is achieved by means of
hierarchical state change transactions. The sub-transaction can be applied to
the blockchain only if everything succeeds, but if the sub-transaction
fails then only its part is rolled back - other changes may still be applied. This
is very similar to how most database systems work.

## Conclusion

So now you have seen the power of the Actor Model to avoid reentrancy, properly
handle errors, and safely sandbox contracts. This helps us provide the solid
security guarantees of the CosmWasm platform. Let’s start playing around
with real contracts in the `wasmd` blockchain!
