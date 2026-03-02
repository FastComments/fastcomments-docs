#### Preuzmite dodatak

Preuzmite najnoviji ZIP iz <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repozitorijuma</a>.

#### Raspakujte u svoj Moodle direktorijum

Raspakujte ZIP u vašu Moodle instalaciju tako da dodatak bude na `<moodle-root>/local/fastcomments`. Direktorijum dodatka treba direktno da sadrži `version.php`, `lib.php`, i druge fajlove dodatka (ne ugnježdene u podfolderu).

Na primer:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalirajte preko Moodle administracije

Prijavite se kao administrator sajta i idite na **Administracija sajta > Obaveštenja**. Moodle će otkriti novi dodatak i zatražiće od vas da pokrenete instalaciju.

#### Konfigurišite dodatak

Nakon instalacije, idite na **Administracija sajta > Dodaci > Lokalni dodaci > FastComments** da unesete svoje postavke. Pogledajte odeljak [Konfiguracija](#moodle-configuration) za detalje o svakoj opciji.

---