#### Get the Plugin

Get the plugin from the <a href="https://moodle.org/plugins/local_fastcomments" target="_blank">Moodle Plugin Directory</a>. You can install it directly from within Moodle's plugin installer, or download the ZIP and install manually.

#### Manual Install: Extract to Your Moodle Directory

If installing manually, extract the ZIP file into your Moodle installation so the plugin resides at `<moodle-root>/local/fastcomments`. The plugin directory should contain `version.php`, `lib.php`, and other plugin files directly (not nested inside a subfolder).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Install via Moodle Admin

Log in as a site administrator and navigate to **Site Administration > Notifications**. Moodle will detect the new plugin and prompt you to run the installation.

#### Configure the Plugin

After installation, go to **Site Administration > Plugins > Local plugins > FastComments** to enter your settings. See the [Configuration](#moodle-configuration) section for details on each option.