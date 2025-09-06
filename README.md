# spacetraders_oc_qwen3
Spacetraders client made with Qwen3 running locally via opencode.

# SpaceTraders Agent Implementation Plan

## 1. Project Setup
- Add necessary dependencies to Cargo.toml (reqwest for HTTP, serde for JSON parsing)
- Create directory structure for modules

## 2. API Client Implementation
- Create a client struct with base URL and authentication header
- Implement methods for:
  - Registering new agents (`POST /api/v2/register`)
  - Getting agent info (`GET /api/v2/my/agent`)
  - Navigation operations (ship movement, waypoint exploration)

## 3. Data Models
- Define structs for API responses:
  - Agent data
  - Ship data
  - System and waypoint information
  - Market data for trading

## 4. Core Game Loop
- Initialize agent with registration 
- Implement main loop that:
  - Checks current ship status and location
  - Explores nearby systems/waypoints
  - Performs actions (navigate, extract, trade)
  - Manages resources/cargo

## 5. Agent Behavior
- Basic strategy for:
  - Finding profitable trade routes
  - Mining resources when needed
  - Moving between systems to maximize efficiency

This plan creates a foundation that can be expanded with more sophisticated AI behavior later.