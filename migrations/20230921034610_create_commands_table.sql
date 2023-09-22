-- Add migration script here
CREATE VIRTUAL TABLE IF NOT EXISTS Commands USING fts5 (
    command_text,
    description,
    tokenize="trigram"
);
