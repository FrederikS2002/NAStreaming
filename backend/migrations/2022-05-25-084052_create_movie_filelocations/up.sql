CREATE TABLE IF NOT EXISTS movie_filelocations(
    `id` INT NOT NULL AUTO_INCREMENT , 
    `uuid` TEXT NOT NULL ,
    `epi` TEXT NOT NULL ,
    `name` TEXT NOT NULL ,
    `location` TEXT NOT NULL ,
    PRIMARY KEY (`id`));
