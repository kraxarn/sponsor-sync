create table if not exists sponsor_time
(
    id         varchar(64) primary key,
    video_id   varchar(14) not null,
    start_time float    not null,
    end_time   float    not null
)