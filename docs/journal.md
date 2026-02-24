# Development Journal

## Day 1

- Initialized project structure
- Added frontend and backend directories
- Added config templates

## Day 2

- Designed initial PostgreSQL schema
- Created roles, users, and applications tables
- Added seed data for testing
- Started documenting database structure
- Introduced ENUM type for application status
- Enforced NOT NULL constraints on workflow fields
- Made seed script idempotent using ON CONFLICT
- Installed PostgreSQL locally and created smart_scholar database
- Executed schema and seed scripts successfully
- Verified tables and sample data in database

## Day 3

- Initialized Rust backend project using Axum
- Configured Tokio runtime and core dependencies
- Implemented base server with `/health` endpoint
- Added structured logging using tracing
- Integrated PostgreSQL using SQLx connection pool
- Implemented environment variable configuration with dotenv
- Added lazy DB connection fallback for reliability
- Created `/health/db` endpoint for database monitoring
- Improved startup error handling and Docker compatibility
- Addressed automated code review feedback
