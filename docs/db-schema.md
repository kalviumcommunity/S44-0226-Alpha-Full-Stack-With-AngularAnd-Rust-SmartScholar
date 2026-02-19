# Smart Scholar – Database Schema Documentation

## Overview

This document describes the PostgreSQL database schema used in the Smart Scholar system.

The database supports:

- User authentication
- Role-based access control
- Scholarship application workflow
- Status tracking

---

## Tables

### 1. roles

Purpose:
Stores system roles that define user permissions.

Fields:

| Column     | Type      | Description            |
| ---------- | --------- | ---------------------- |
| id         | SERIAL PK | Unique role identifier |
| name       | VARCHAR   | Role name (unique)     |
| created_at | TIMESTAMP | Creation timestamp     |

Example Records:

| id  | name     |
| --- | -------- |
| 1   | STUDENT  |
| 2   | VERIFIER |
| 3   | APPROVER |
| 4   | FINANCE  |
| 5   | ADMIN    |

---

### 2. users

Purpose:
Stores registered users of the system.

Fields:

| Column        | Type      | Description            |
| ------------- | --------- | ---------------------- |
| id            | SERIAL PK | Unique user ID         |
| full_name     | VARCHAR   | User full name         |
| email         | VARCHAR   | Login email (unique)   |
| password_hash | TEXT      | Encrypted password     |
| role_id       | INTEGER   | Reference to roles(id) |
| created_at    | TIMESTAMP | Account creation time  |

Example Records:

| id  | full_name    | email            | role_id |
| --- | ------------ | ---------------- | ------- |
| 1   | Test Student | student@test.com | 1       |

---

### 3. applications

Purpose:
Stores scholarship applications submitted by students.

Fields:

| Column           | Type               | Description             |
| ---------------- | ------------------ | ----------------------- |
| id               | SERIAL PK          | Application ID          |
| user_id          | INTEGER            | Reference to users(id)  |
| scholarship_name | VARCHAR            | Scholarship name        |
| status           | application_status | Current workflow status |
| submitted_at     | TIMESTAMP          | Submission timestamp    |
| updated_at       | TIMESTAMP          | Last modification time  |

Status Values:

- PENDING
- VERIFIED
- APPROVED
- REJECTED

Example Records:

| id  | user_id | scholarship_name | status  |
| --- | ------- | ---------------- | ------- |
| 1   | 1       | Merit Grant      | PENDING |

---

## Relationships

- One role → many users
- One user → many applications

### Foreign Keys

| Table        | Column  | References | Description                  |
| ------------ | ------- | ---------- | ---------------------------- |
| users        | role_id | roles(id)  | Assigns role to user         |
| applications | user_id | users(id)  | Links application to student |

Relationship Rules:

- One role can be assigned to many users
- One user can submit multiple applications
- Each application belongs to exactly one user

---

## Constraints and Validation

- Role names are unique
- User emails are unique
- Foreign keys enforce data integrity
- Status uses ENUM for workflow safety
- Workflow timestamps are NOT NULL
- Seed scripts are idempotent

---

## Notes

- Schema supports multi-level approval workflow
- Designed for future scalability
- Suitable for enterprise-style deployment
