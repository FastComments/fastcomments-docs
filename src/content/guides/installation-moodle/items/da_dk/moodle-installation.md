#### Hent plugin

Hent den seneste release-ZIP fra <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub-repositorium</a>.

#### Udpak til din Moodle-mappe

Udpak ZIP-filen til din Moodle-installation, så pluginet ligger i `<moodle-root>/local/fastcomments`. Pluginmappen bør indeholde `version.php`, `lib.php`, og andre pluginfiler direkte (ikke indlejret i en undermappe).

For eksempel:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Installer via Moodle-administrator

Log ind som en siteadministrator og gå til **Site Administration > Notifications**. Moodle vil registrere det nye plugin og bede dig om at køre installationen.

#### Konfigurer plugin

Efter installationen, gå til **Site Administration > Plugins > Local plugins > FastComments** for at indtaste dine indstillinger. Se afsnittet [Konfiguration](#moodle-configuration) for detaljer om hver indstilling.