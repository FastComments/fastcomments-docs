#### Download the Plugin

Download the latest release ZIP file from the <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repository</a>.

#### Extract to Your Moodle Directory

Extract the ZIP file into your Moodle installation so the plugin resides at `<moodle-root>/local/fastcomments`. The plugin directory should contain `version.php`, `lib.php`, and other plugin files directly (not nested inside a subfolder).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Install via Moodle Admin

Log in as a site administrator and navigate to **Site Administration > Notifications**. Moodle will detect the new plugin and prompt you to run the installation.

#### Configure the Plugin

After installation, go to **Site Administration > Plugins > Local plugins > FastComments** to enter your settings. See the [Configuration](#items-moodle-configuration) section for details on each option.