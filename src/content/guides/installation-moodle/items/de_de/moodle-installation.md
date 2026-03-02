#### Plugin herunterladen

Laden Sie die neueste Release-ZIP aus dem <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub-Repository</a> herunter.

#### In Ihr Moodle-Verzeichnis entpacken

Entpacken Sie die ZIP in Ihre Moodle-Installation, sodass das Plugin unter `<moodle-root>/local/fastcomments` liegt. Das Plugin-Verzeichnis sollte direkt `version.php`, `lib.php` und andere Plugin-Dateien enthalten (nicht in einem Unterordner verschachtelt).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Installation über Moodle-Administration

Melden Sie sich als Website-Administrator an und navigieren Sie zu **Site Administration > Notifications**. Moodle erkennt das neue Plugin und fordert Sie auf, die Installation auszuführen.

#### Plugin konfigurieren

Nach der Installation gehen Sie zu **Site Administration > Plugins > Local plugins > FastComments**, um Ihre Einstellungen vorzunehmen. Siehe den [Konfiguration](#moodle-configuration)-Abschnitt für Details zu jeder Option.

---