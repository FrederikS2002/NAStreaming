CREATE TABLE IF NOT EXISTS movie_filelocations(
    `id` INT NOT NULL AUTO_INCREMENT ,
    `uuid` TEXT NOT NULL ,
    `movie` TEXT NOT NULL ,
    `epi` TEXT NOT NULL ,
    `name` TEXT NOT NULL ,
    `filename` TEXT NOT NULL ,
    PRIMARY KEY (`id`));
