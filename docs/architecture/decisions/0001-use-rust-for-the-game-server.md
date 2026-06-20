# Use Rust for the game server

## Context and Problem Statement

I needed to build an MMO game server capable of supporting thousands of simultaneously playing players. The programming language chosen would have a big impact on the server's performance and maintainability.

## Considered Options

- C++
- Rust

## Decision Outcome

Although C++ is the more conventional language for game engine development, and despite being faster at developing in C++ than Rust due to the Rust compiler's strictness, I chose to build the server in Rust for the extra safety guarantees. I'm betting that the tradeoff of investing extra effort up front for safety will pay off when I'm maintaining the server as a solo developer.