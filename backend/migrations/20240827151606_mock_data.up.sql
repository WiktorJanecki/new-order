-- Add up migration script here
INSERT INTO users
(name, password,privileges)
VALUES 
('worker', '123', 'Basic'),
('boss', '123', 'Full');
