CREATE TABLE IF NOT EXISTS movie_watchtime(
    `id` INT NOT NULL AUTO_INCREMENT , 
    `user` TEXT NOT NULL ,
    `watchtime` INT NOT NULL ,
    PRIMARY KEY (`id`));
