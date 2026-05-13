# Soroban Service Escrow

A Stellar Soroban smart contract for secure service payments between clients and specialists.

## Problem

Freelancers, contractors, and service providers often face trust issues:

- Clients fear paying before work is delivered
- Specialists fear completing work without payment
- Disputes are hard to resolve fairly

## Solution

This project provides an escrow smart contract where:

1. A client creates a job
2. The client funds the escrow
3. A specialist completes the work
4. The client approves the work
5. Funds are released to the specialist
6. Disputes can be resolved by an admin

## Core Features

- Create escrow job
- Fund escrow
- Submit work
- Approve work
- Release payment
- Raise dispute
- Resolve dispute
- Refund client

## Tech Stack

- Rust
- Stellar Soroban SDK
- Smart contracts
- Unit tests

## Maintainer Goal

This repository is designed for open-source contributors to help improve smart contract logic, tests, documentation, and frontend/backend integrations.
