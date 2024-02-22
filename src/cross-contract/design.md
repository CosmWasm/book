# Design

This time we will start discussing the design of our system a bit. Building multi-contract systems tend to
be a bit more complicated than just isolated contracts, so I want to give you some anchor on what we are
building in this chapter. If you feel lost with a design, don't worry - it will get clear while implementing
contracts. For now, go through it to get a general idea.

First, let's think about the problem we want to solve. Our admins are a vector of addresses. Anyone already
an admin can add anyone he wants to the list. But this "anyone" can be a second instance of the same admin
account, so he counts twice for donations!

This issue is relatively simple to fix, but there is another problem - as we already learned, the admin could
create a smart contract which he and only he can withdraw tokens from and register as another admin in the
group! Instantiating it multiple times, he can achieve his goal even if we prevent adding the same address
multiple times. There would be many distinct addresses that the same person owns.

It looks like an unpleasant situation, but there are ways to manage it. The one we would implement is voting.
Instead of being able to add another admin to the list, admins would be allowed to propose their colleagues
as new admins. It would start a voting process - everyone who was an admin at the time of the proposal creation
would be able to support it. If more than half admins would support the new candidate, he would immediately
become an admin.

It is not the most convoluted voting process, but it would be enough for our purposes.

## Voting process

To achieve this goal, we would create two smart contracts. First, one would be reused contract from the
[Basics](../basics.md) chapter - it would be an `admin` contract. Additionally, we would add a `voting` contract.
It would be responsible for managing a single voting process. It would be instantiated by an `admin` contract
whenever an admin wants to add his friend to a list. Here is a diagram of the contracts relationship:

```plantuml
@startuml
class admin {
    admins: Map<Addr, Timestamp>
    votings: Map<Addr, Addr>

    propose_admin(candidate: Addr)
    add_admin()
    leave()
    donate()

    admins_list() -> Vec<Addr>
    join_time() -> Timestamp
}

class voting {
    votes: Vec<Addr>
    votes_needed: u64
    closed: bool

    accept()
    votes_list() -> Vec<Addr>
}

admin o- voting: manages
@enduml
```

Here is adding an admin flowchart - assuming there are 5 admins on the contract already, but 2 of them did nothing:

```plantuml
@startuml
actor "Admin 1" as admin1
actor "Admin 2" as admin2
actor "Admin 3" as admin3
entity "Admin Contract" as admin
entity "Votes" as votes

admin1 -> admin: exec propose_admin { addr: new_admin }

admin -> votes **: instantiate { addr: "new_admin", required: 3 }

admin2 -> votes ++: exec accept {}
votes -> admin: query join_time { admin: "admin2" }
admin -> votes: resp join_time_resp { joined: ... }
votes --> votes --: add vote

admin3 -> votes ++: exec accept {}
votes -> admin: query join_time { admin: "admin3" }
admin -> votes: resp join_time_resp { joined: ... }
votes --> votes: add vote

votes -> admin --: add_admin { addr: new_admin }

@enduml
```

I already put some hints about contracts implementation, but I will not go into them yet.

## Messages forwarding

There is one other thing we want to add - some way to give admins work. The `admin` contract would behave like
a proxy to call another contract. That means that some other external contract would just set our `admin` instance
as a specific address that can perform executions on it, and admins would perform actions this way. The external
contract would see execution as the admin contract would do it. Here is an updated contracts diagram:

```plantuml
@startuml
class admin {
    admins: Map<Addr, Timestamp>
    votings: Map<Addr, Addr>

    propose_admin(candidate: Addr)
    add_admin()
    leave()
    donate()
    execute(contract: Addr, message: Binary)

    admins_list() -> Vec<Addr>
    join_time() -> Timestamp
}

class voting {
    votes: Vec<Addr>
    votes_needed: u64
    closed: bool

    accept()
    votes_list() -> Vec<Addr>
}

class external {}

admin o- voting: manages
admin -- external: forwards to
@enduml
```

And calling external contract flowchart:

```plantuml
@startuml
actor Admin as admin
entity "Admin Contract" as contract
entity "External Contract" as external

admin -> contract ++: exec execute { addr: "external_contract", msg: message_to_execute }
contract -> external ++: exec message_to_execute
deactivate external
deactivate contract
@enduml
```

Note that the `msg` on `execute` admin contract message is some arbitrary message just forwarded
to the external contract. It would be a base64-encoded message in the real world, but it is
just an implementation detail.

Ultimately, we will create a simple example of an external contract to understand how to use such a pattern.
