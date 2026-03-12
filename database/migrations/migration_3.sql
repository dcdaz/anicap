ALTER TABLE serie 
    ADD COLUMN watch_status TINYINT DEFAULT 0 CHECK (watch_status IN (0, 1, 2));