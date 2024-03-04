# Basics

In this chapter, we will go through creating essential smart contracts step by step. I will explain the core ideas behind CosmWasm and the typical contract structure.

### **Introduction to CosmWasm**

**How to Understand Core Concepts:**

- **Wasm**: Experiment with compiling simple Rust programs to WebAssembly to understand the compilation process. Tools like **`wasm-pack`** can help in this learning phase.
- **Interoperability**: Explore the Cosmos SDK documentation and IBC protocol to grasp how CosmWasm facilitates cross-chain interactions. Look for simple IBC examples or tutorials to see this in action.
- **Rust Programming Language**: If new to Rust, start with "The Rust Programming Language" book (often referred to as "The Book") available for free online. Rust's ownership model and safety guarantees are crucial for writing secure smart contracts.

### **Typical Contract Structure**

1. **Contract Entry Points**
    
    **How to Implement Entry Points:**
    
    - **Instantiate**: Begin by defining what initial state your contract will have. Implement the **`instantiate`** function to set initial values, ensuring to handle any necessary input validation.
    - **Execute**: Identify the actions your contract must support. Implement each action as a case within the **`execute`** function, manipulating the contract's state as required.
    - **Query**: Determine the data users might need to read from your contract. Implement the **`query`** function to return this data, ensuring no state modification occurs.
2. **Messages**
    
    **How to Define Messages:**
    
    - Use Rust's **`enum`** and **`struct`** to define messages. Each action your contract supports will be a variant in the **`ExecuteMsg`** enum. Similarly, define **`InstantiateMsg`** and **`QueryMsg`** based on what data they must carry.
3. **State Management**
    
    **How to Manage State:**
    
    - Design your state model carefully, considering how data is accessed and updated. Use the **`DepsMut`** for state mutations and **`Deps`** for state queries. Implement helper functions for common state operations to encapsulate the storage access patterns.
4. **Dependencies**
    
    **How to Handle Dependencies:**
    
    - Familiarize yourself with the **`cosmwasm_std`** library's API by reading the documentation and looking at example contracts. For external crates, ensure they support compilation to **`wasm32-unknown-unknown`** target.

### **Step-by-Step Guide to Creating a Smart Contract**

1. **Setup**
    - Install Rust and **`cargo`** using rustup. Set up your IDE for Rust development (VSCode, IntelliJ, etc.). Create a new project using **`cargo new --lib your_contract_name`**.
2. **Define Messages**
    - Sketch out the functionality of your contract. For each function, define a message in Rust, using structs for data and enums for differentiating message types.
3. **Implement Entry Points**
    - Start with the **`instantiate`** function for setting the initial state. Proceed to **`execute`** where you'll handle different messages as per your contract's logic. Finally, implement **`query`** for data retrieval.
4. **State Management**
    - Use the **`cosmwasm_std::Storage`** trait for state management. Design your key-value schema and implement getter and setter functions for interacting with the state.
5. **Business Logic**
    - Flesh out the business logic within the **`execute`** function. Consider edge cases and input validation to ensure contract security and reliability.
6. **Testing**
    - Write unit tests for each entry point and critical functions within your contract. Use **`cargo test`** to run your tests. Consider edge cases and invalid inputs to ensure your contract behaves as expected under various conditions.