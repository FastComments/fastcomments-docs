#### Hent plugin'et

Hent den seneste udgivelses-ZIP fra <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repository</a>.

#### Udpak til din Moodle-mappe

Udpak ZIP-filen ind i din Moodle-installation, så plugin'et ligger på `<moodle-root>/local/fastcomments`. Plugin-mappen bør indeholde `version.php`, `lib.php` og andre plugin-filer direkte (ikke indlejret i en undermappe).

For eksempel:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Installer via Moodle-administrator

Log ind som siteadministrator og gå til **Site Administration > Notifications**. Moodle vil registrere det nye plugin og bede dig om at køre installationen.

#### Konfigurer plugin'et

Efter installationen skal du gå til **Site Administration > Plugins > Local plugins > FastComments** for at indtaste dine indstillinger. Se sektionen [Konfiguration](#items-moodle-configuration) for detaljer om hver indstilling.