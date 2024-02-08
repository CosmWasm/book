# Execution Messages in CosmWasm 2.0

## Introduction to Execution Messages

Execution messages in CosmWasm smart contracts serve as the core mechanism for performing state-altering actions. These messages enable a broad spectrum of functionalities, ranging from simple administrative adjustments to the implementation of complex business logic within the contract.

## Setting the Stage
Execution messages play a pivotal role in smart contract administration, facilitating essential governance actions such as:

- **Adding members** to the administrator list.
- **Allowing administrators** to leave or remove themselves from the list.
This section explores how to manage contract administrators effectively through execution messages.

## Defining Execution Messages
Execution messages are defined within the ExecuteMsg enum, providing a structured approach to specifying administrative actions: