# Use C++ and Python for the quant trading system

## Context and Problem Statement

I want to build a quantitative trading system for preserving Stirge Hunters income. The programming language selected will have a significant impact on system performance.

## Considered Options

- C++
- Rust
- Python

## Decision Outcome

Although Rust adds important safety guarantees that C++ can't make, C++ is better integrated into the Python ecosystem where many mature tools for data science live. I've opted to build the trading system using a combination of C++ for real-time processing and Python for analysis.