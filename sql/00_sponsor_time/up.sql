create table if not exists sponsor_time
(
    id         uuid primary key,
    video_id   char(11) not null,
    start_time float    not null,
    end_time   float    not null
)