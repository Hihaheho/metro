# Metro Engine

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/metro-agent)](https://crates.io/crates/metro-agent)
[![docs.rs](https://img.shields.io/docsrs/metro-agent)](https://docs.rs/metro-agent)
[![GitHub Repo stars](https://img.shields.io/github/stars/Hihaheho/metro?style=social)](https://github.com/Hihaheho/metro)

An intelligent agent engine with graph-structured blackboard with type-safe
query system.

**Status: WIP to release the first working version 0.1.0**

## Overview

Metro Engine provides:

- Predefined entities that can be included in a blackboard.
- Predefined agent units: units are reusable and composable data
  types for building an agent from scratch.
- An official, opinionated agent runtime.

If you don't need them, you can use the following internal packages directly from your project:

- [metro-blackboard](https://github.com/Hihaheho/metro/tree/main/crates/metro-blackboard#metro-blackboard)
  for the blackboard data structure and its query system.
- [metro-agent](https://github.com/Hihaheho/metro/tree/main/crates/metro-agent#metro-agent)
  for runtime agnostic agent builder based on `metro-blackboard`.

## Features

- Ergonomic API with minimal boilerplate.
- Versatility across various use cases and high-level architectural designs of agents.
- Simplified agent construction through type-safe composition of reusable agent units.
- Type-safe blackboard featuring a graph data structure for complex data management.
- Efficient, typed query system for optimized data retrieval.
- First-class support for type-safe dynamic operations, such as (de)serializing or inspection.
