-- Create the events table
CREATE TABLE `events` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `timestamp` int(20) NOT NULL DEFAULT 0,
  `session_id` varchar(256) NOT NULL DEFAULT '',
  `time_on_page` int(11) DEFAULT NULL,
  `username` varchar(256) NOT NULL DEFAULT 'unknown',
  `event_type` varchar(64) NOT NULL DEFAULT 'noop',
  `hostname` varchar(256) NOT NULL DEFAULT '',
  `pathname` varchar(2048) NOT NULL DEFAULT '',
  `search` varchar(2048) DEFAULT NULL,
  `in_focus` tinyint(1) NOT NULL DEFAULT '1',
  `prior_hostname` varchar(256) DEFAULT NULL,
  `prior_pathname` varchar(2048) DEFAULT NULL,
  `prior_search` varchar(2048) DEFAULT NULL,
  PRIMARY KEY (`id`)
)