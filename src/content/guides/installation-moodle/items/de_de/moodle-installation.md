#### Plugin herunterladen

Laden Sie die neueste Release-ZIP aus dem <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub-Repository</a> herunter.

#### In Ihr Moodle-Verzeichnis entpacken

Entpacken Sie die ZIP-Datei in Ihre Moodle-Installation, sodass das Plugin unter `<moodle-root>/local/fastcomments` liegt. Das Plugin-Verzeichnis sollte direkt `version.php`, `lib.php` und andere Plugin-Dateien enthalten (nicht in einem Unterordner verschachtelt).

Zum Beispiel:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Über das Moodle-Admin installieren

Melden Sie sich als Website-Administrator an und navigieren Sie zu **Website-Administration > Benachrichtigungen**. Moodle erkennt das neue Plugin und fordert Sie auf, die Installation auszuführen.

#### Plugin konfigurieren

Nach der Installation gehen Sie zu **Website-Administration > Plugins > Lokale Plugins > FastComments**, um Ihre Einstellungen vorzunehmen. Siehe den Abschnitt [Konfiguration](#moodle-configuration) für Details zu jeder Option.