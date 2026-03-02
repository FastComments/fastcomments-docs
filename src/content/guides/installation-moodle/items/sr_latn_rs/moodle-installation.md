#### Preuzmite plugin

Preuzmite najnoviji ZIP sa izdanjem sa <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repozitorijuma</a>.

#### Otpakujte u vaš Moodle direktorijum

Otpakujte ZIP u vašu Moodle instalaciju tako da se plugin nalazi na `<moodle-root>/local/fastcomments`. Direktorijum plugina treba direktno da sadrži `version.php`, `lib.php`, i druge fajlove plugina (ne ugnježdene u podfolderu).

Na primer:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalirajte putem Moodle administracije

Prijavite se kao administrator sajta i idite na **Administracija sajta > Obaveštenja**. Moodle će otkriti novi plugin i zatražiti da pokrenete instalaciju.

#### Konfigurišite plugin

Nakon instalacije, idite na **Administracija sajta > Dodaci > Lokalni dodaci > FastComments** da unesete vaše postavke. Pogledajte odeljak [Konfiguracija](#moodle-configuration) za detalje o svakoj opciji.