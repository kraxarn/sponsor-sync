create table if not exists sponsor_time
(
    id         varchar(96) primary key,
    video_id   varchar(16) not null,
    start_time float       not null,
    end_time   float       not null
)