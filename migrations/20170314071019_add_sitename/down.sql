-- This file should undo anything in `up.sql`
USE `hoppler`;
ALTER TABLE `events` DROP `site_name`;
ALTER TABLE `events` DROP `time_at_focus_state`;

