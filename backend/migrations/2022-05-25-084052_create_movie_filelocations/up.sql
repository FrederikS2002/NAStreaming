CREATE TABLE IF NOT EXISTS movie_filelocations(
    `id` INT NOT NULL AUTO_INCREMENT ,
    `uuid` TEXT NOT NULL ,
    `movie` TEXT NOT NULL ,
    `name` TEXT NOT NULL ,
    `epi` INT NOT NULL ,
    `description` TEXT NOT NULL,
    `filename` TEXT NOT NULL ,
    PRIMARY KEY (`id`));
