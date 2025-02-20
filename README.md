# Socrates, the Lightweight Persistence Layer

The goal of this proof of concept project is to implement an effective persistence layer without any external dependency like a database server.

The implementation is based on the Command Query Responsibility Segregation (CQRS) and Event Sourcing patterns.

## Concept

Using this package a developer don't need to implement anything related to persistence. An in-memory aggregate implementation is needed only with commands and queries. The incoming commands are used to update the aggregate in the memory, but they are stored in a persistent way as well. If the process is restarted, then the already stored events are executed as commands to build app the state of the aggregate again.

## Benefits

- Persistence implementation is very easy and fast: It is the same as for storing data in the memory only
- The whole functionality of persistence can be easily unit-tested
- Incrediable performance (using the memory only for queries)
- Very easy upgrade (As long as the events remains compatible, no any upgrade speicfic implementation is needed)
- Easy and effective concurrency handling without deadlocks: Queries can executed in parallel, commands lock the aggregate for very short period of time to do in-memory operations only

The only disadvantage is the high memory usage, but memory modules are evolving fast. There are VMs in the cloud even with terabytes of memories.
