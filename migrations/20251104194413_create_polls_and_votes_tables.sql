-- Create polls table
CREATE TABLE polls (
                       id SERIAL PRIMARY KEY,
                       question_text VARCHAR(255) NOT NULL,
                       option_a VARCHAR(100) NOT NULL,
                       option_b VARCHAR(100) NOT NULL,
                       created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create votes table
CREATE TABLE votes (
                       id SERIAL PRIMARY KEY,
                       poll_id INTEGER NOT NULL REFERENCES polls(id),
                       chosen_option VARCHAR(100) NOT NULL,
                       voted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);