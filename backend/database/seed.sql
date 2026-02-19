-- Insert Roles (idempotent)
INSERT INTO roles (name) VALUES
('STUDENT'),
('VERIFIER'),
('APPROVER'),
('FINANCE'),
('ADMIN')
ON CONFLICT (name) DO NOTHING;

-- Insert Sample User (idempotent)
INSERT INTO users (full_name, email, password_hash, role_id)
SELECT
    'Test Student',
    'student@test.com',
    'hashed_password_here',
    r.id
FROM roles r
WHERE r.name = 'STUDENT'
ON CONFLICT (email) DO NOTHING;
