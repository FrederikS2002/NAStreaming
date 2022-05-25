CREATE TABLE IF NOT EXISTS movie_progress(
    `id` INT NOT NULL AUTO_INCREMENT , 
    `uuid` TEXT NOT NULL ,
    `epi` TEXT NOT NULL ,
    `progress` INT NOT NULL ,
    `user` TEXT NOT NULL ,
    PRIMARY KEY (`id`));

