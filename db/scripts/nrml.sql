insert into users (username,password,email) values ('admin','123','admin@email.com');
insert into tags (name) values('osaka(azumanga_daioh)'),('azumanga_daioh'),('lain(serial_experiments_lain)'),('serial_experiments_lain');
insert into tags (name) values('gif');

insert into posts (uploader,artist,tags) values ('admin','unknow',ARRAY['osaka(azumanga_daioh)','azumanga_daioh'] );
insert into posts (uploader,artist,tags) values ('admin','unknow',ARRAY['osaka(azumanga_daioh)','azumanga_daioh','gif'] );
insert into posts (uploader,artist,tags) values ('admin','unknow',ARRAY['lain(serial_experiments_lain)','serial_experiments_lain'] );

select * from users;
select * from posts;
select * from tags;



SELECT id, uploader, artist, tags
FROM posts
WHERE tags @> ARRAY[]::text[];


update tags set tag_type = 'C' where "name" = 'lain(serial_experiments_lain)';
select * from tags;


insert into tags(name,tag_type) values
	('kasane_teto', 'C'),
	('Video', 'V'),
	('song', 'd');



select COUNT(*) FROM posts

delete from posts where id = '7';


SELECT setval('posts_id_seq', COALESCE((SELECT MAX(id) FROM posts), 0) + 1, false);
SELECT setval('posts_id_seq', (SELECT COALESCE(MAX(id), 0) FROM posts));



SELECT MAX(id) FROM posts;

SELECT last_value + 1 FROM posts_id_seq;

