-- Create the events table
CREATE TABLE `events` (
  `id` integer unsigned NOT NULL AUTO_INCREMENT,
  `timestamp` bigint NOT NULL DEFAULT 0,
  `session_id` varchar(256) NOT NULL DEFAULT '',
  `time_on_page` integer DEFAULT NULL,
  `username` varchar(256) NOT NULL DEFAULT 'unknown',
  `event_type` varchar(64) NOT NULL DEFAULT 'noop',
  `hostname` varchar(256) NOT NULL DEFAULT '',
  `pathname` varchar(2048) NOT NULL DEFAULT '',
  `search` varchar(2048) DEFAULT NULL,
  `in_focus` tinyint NOT NULL DEFAULT '1',
  `prior_hostname` varchar(256) DEFAULT NULL,
  `prior_pathname` varchar(2048) DEFAULT NULL,
  `prior_search` varchar(2048) DEFAULT NULL,
  PRIMARY KEY (`id`)
)