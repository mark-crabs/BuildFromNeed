CREATE TABLE solution (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    problem_id INT NOT NULL REFERENCES problem(id) ON DELETE CASCADE,
    solution_type TEXT NOT NULL CHECK (solution_type IN ('Solution', 'Comment')) DEFAULT 'Comment',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
