---
#### Preuzmite dodatak

Preuzmite najnoviji ZIP izdanja sa <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repozitorija</a>.

#### Raspakujte u vašu Moodle direktorijum

Raspakujte ZIP u vašu Moodle instalaciju tako da se dodatak nalazi na `<moodle-root>/local/fastcomments`. Direktorijum dodatka treba sadržavati `version.php`, `lib.php`, i druge datoteke dodatka direktno (ne ugnježdene u podfolderu).

Na primjer:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalirajte putem Moodle administratora

Prijavite se kao administrator sajta i idite na **Administracija sajta > Obavještenja**. Moodle će otkriti novi dodatak i zatražiti od vas da pokrenete instalaciju.

#### Konfigurišite dodatak

Nakon instalacije, idite na **Administracija sajta > Dodaci > Lokalni dodaci > FastComments** da unesete vaše postavke. Pogledajte odjeljak [Konfiguracija](#moodle-configuration) za detalje o svakoj opciji.

---