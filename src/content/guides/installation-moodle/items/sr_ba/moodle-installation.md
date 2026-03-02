#### Preuzmite dodatak

Preuzmite najnoviji ZIP izdanja sa <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repozitorij</a>.

#### Otpakujte u Moodle direktorijum

Otpakujte ZIP u vašu Moodle instalaciju tako da se dodatak nalazi na `<moodle-root>/local/fastcomments`. Direktorijum dodatka treba da sadrži `version.php`, `lib.php`, i druge datoteke dodatka direktno (ne u podfolderu).

Na primjer:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalirajte putem Moodle administracije

Prijavite se kao administrator sajta i idite na **Site Administration > Notifications**. Moodle će otkriti novi dodatak i zatražiti da pokrenete instalaciju.

#### Konfigurišite dodatak

Nakon instalacije, idite na **Site Administration > Plugins > Local plugins > FastComments** da unesete svoje postavke. Pogledajte odjeljak [Konfiguracija](#moodle-configuration) za detalje o svakoj opciji.