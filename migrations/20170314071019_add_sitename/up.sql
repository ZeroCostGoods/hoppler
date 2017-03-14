USE `hoppler`;
ALTER TABLE `events` ADD `time_at_focus_state` INT(11)  NULL  DEFAULT NULL  AFTER `time_on_page`;
ALTER TABLE `events` ADD `site_name` VARCHAR(256)  NOT NULL  DEFAULT ''  AFTER `time_on_page`;
