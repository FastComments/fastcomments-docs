#### Preuzmite dodatak

Preuzmite najnoviji ZIP paket s <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repozitorija</a>.

#### Raspakirajte u direktorij vaše Moodle instalacije

Raspakirajte ZIP u svoju Moodle instalaciju tako da je dodatak smješten u `<moodle-root>/local/fastcomments`. Direktorij dodatka trebao bi izravno sadržavati `version.php`, `lib.php`, i druge datoteke dodatka (ne ugniježđene u podmapu).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalirajte putem Moodle administracije

Prijavite se kao administrator stranice i idite na **Site Administration > Notifications**. Moodle će otkriti novi dodatak i zatražiti od vas da pokrenete instalaciju.

#### Konfigurirajte dodatak

Nakon instalacije, idite na **Site Administration > Plugins > Local plugins > FastComments** kako biste unijeli svoje postavke. Pogledajte odjeljak [Konfiguracija](#moodle-configuration) za detalje o svakoj opciji.