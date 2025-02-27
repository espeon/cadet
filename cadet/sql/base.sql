CREATE TABLE artists (
    mbid UUID PRIMARY KEY,
    name TEXT NOT NULL,
    play_count INTEGER DEFAULT 0
);

-- releases are synologous to 'albums'
CREATE TABLE releases (
    mbid UUID PRIMARY KEY,
    name TEXT NOT NULL,
    play_count INTEGER DEFAULT 0
);

-- recordings are synologous to 'tracks' BUT tracks can be in multiple releases!
CREATE TABLE recordings (
    mbid UUID PRIMARY KEY,
    name TEXT NOT NULL,
    play_count INTEGER DEFAULT 0
);

CREATE TABLE plays (
    uri TEXT PRIMARY KEY,
    did TEXT NOT NULL,
    rkey TEXT NOT NULL,
    cid TEXT NOT NULL,
    isrc TEXT,
    duration INTEGER,
    track_name TEXT NOT NULL,
    played_time TIMESTAMP
    WITH
        TIME ZONE,
        processed_time TIMESTAMP
    WITH
        TIME ZONE,
        release_mbid UUID,
        release_name TEXT,
        recording_mbid UUID,
        submission_client_agent TEXT,
        music_service_base_domain TEXT,
        FOREIGN KEY (release_mbid) REFERENCES releases (mbid),
        FOREIGN KEY (recording_mbid) REFERENCES recordings (mbid)
);

CREATE INDEX idx_plays_release_mbid ON plays (release_mbid);

CREATE INDEX idx_plays_recording_mbid ON plays (recording_mbid);

CREATE INDEX idx_plays_played_time ON plays (played_time);

CREATE TABLE play_to_artists (
    play_uri TEXT, -- references plays(uri)
    artist_mbid UUID REFERENCES artists (mbid),
    artist_name TEXT, -- storing here for ease of use when joining
    PRIMARY KEY (play_uri, artist_mbid),
    FOREIGN KEY (play_uri) REFERENCES plays (uri)
);

CREATE INDEX idx_play_to_artists_artist ON play_to_artists (artist_mbid);

-- Materialized view for artists' play counts
CREATE MATERIALIZED VIEW mv_artist_play_counts AS
SELECT
    a.mbid AS artist_mbid,
    a.name AS artist_name,
    COUNT(p.uri) AS play_count
FROM
    artists a
    LEFT JOIN play_to_artists pta ON a.mbid = pta.artist_mbid
    LEFT JOIN plays p ON p.uri = pta.play_uri
GROUP BY
    a.mbid,
    a.name;

CREATE UNIQUE INDEX idx_mv_artist_play_counts ON mv_artist_play_counts (artist_mbid);

-- Materialized view for releases' play counts
CREATE MATERIALIZED VIEW mv_release_play_counts AS
SELECT
    r.mbid AS release_mbid,
    r.name AS release_name,
    COUNT(p.uri) AS play_count
FROM
    releases r
    LEFT JOIN plays p ON p.release_mbid = r.mbid
GROUP BY
    r.mbid,
    r.name;

CREATE UNIQUE INDEX idx_mv_release_play_counts ON mv_release_play_counts (release_mbid);

-- Materialized view for recordings' play counts
CREATE MATERIALIZED VIEW mv_recording_play_counts AS
SELECT
    rec.mbid AS recording_mbid,
    rec.name AS recording_name,
    COUNT(p.uri) AS play_count
FROM
    recordings rec
    LEFT JOIN plays p ON p.recording_mbid = rec.mbid
GROUP BY
    rec.mbid,
    rec.name;

CREATE UNIQUE INDEX idx_mv_recording_play_counts ON mv_recording_play_counts (recording_mbid);

-- global play count
CREATE MATERIALIZED VIEW mv_global_play_count AS
SELECT
    1 as id,
    COUNT(uri) as play_count
FROM
    plays;

CREATE UNIQUE INDEX idx_mv_global_play_count ON mv_global_play_count (id);
