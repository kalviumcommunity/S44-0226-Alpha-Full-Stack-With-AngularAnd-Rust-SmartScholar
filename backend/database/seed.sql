-- Insert Roles
INSERT INTO roles (name) VALUES
('STUDENT'),
('VERIFIER'),
('APPROVER'),
('FINANCE'),
('ADMIN');

-- Sample User
INSERT INTO users (full_name, email, password_hash, role_id)
VALUES (
    'Test Student',
    'student@test.com',
    'hashed_password_here',
    1
);
