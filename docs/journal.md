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

## Day 4

- Initialized Angular frontend project with routing and CSS styling
- Configured environment files for development and production
- Set up base application structure using standalone components
- Created layout components (Navbar, Sidebar, Footer, Main Layout)
- Implemented responsive full-height dashboard layout
- Applied dark and blue professional theme across the application
- Cleanouted default Angular starter template
- Improved global styling and layout consistency
- Addressed frontend code review feedback
- Pushed frontend setup and layout UI branches
