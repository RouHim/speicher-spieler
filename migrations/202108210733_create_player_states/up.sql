CREATE TABLE player_states
(
    id                INTEGER PRIMARY KEY,
    playing_file_path TEXT    NOT NULL,
    playing_file_type TEXT    NOT NULL,
    caching_url       TEXT    NOT NULL,
    queueing_urls     TEXT    NOT NULL,
    player_playing    BOOLEAN NOT NULL DEFAULT 'f'
)