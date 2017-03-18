# Hoppler Server

## Introduction

Hoppler gives user experience designers an insight into how users move within and across multiple sites and apps.

This project is still in an early phase as as work in progress.

## Progress Report

At present, the hoppler server will respond to event posts from the HopplerJS client and store that information in
the database.

### TODO

1. Tweak Rocket logs in production mode.  Is there a way to configure rocket logs without the use of an environment
variable?

2. Add support for queries.  A design doc will detail what kinds of queries we need to support.

3. Add frontend to view and query data.  Again, the design doc will offer details.

## Configuration

1. Database.  Configure a valid connection string in the `config/config.yaml` file
2. Rocket logs.  Configure rocket logs in the `Rocket.toml` file.

