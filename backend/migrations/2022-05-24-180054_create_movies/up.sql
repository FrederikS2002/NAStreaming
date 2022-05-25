CREATE TABLE IF NOT EXISTS `movies` ( 
    `id` INT NOT NULL AUTO_INCREMENT , 
    `uuid` TEXT NOT NULL ,
    `type` TEXT NOT NULL ,
    `titles` TEXT NOT NULL ,
    `categories` TEXT NOT NULL , 
    `age_restriction` INT NOT NULL ,
    `img_src` TEXT NOT NULL ,  
    PRIMARY KEY (`id`));

