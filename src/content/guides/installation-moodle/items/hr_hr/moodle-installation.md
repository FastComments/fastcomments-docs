#### Preuzmite dodatak

Preuzmite najnoviji ZIP iz <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repozitorij</a>.

#### Raspakirajte u svoj Moodle direktorij

Raspakirajte ZIP u svoju Moodle instalaciju tako da dodatak bude smješten na `<moodle-root>/local/fastcomments`. Direktorij dodatka trebao bi sadržavati `version.php`, `lib.php`, i druge datoteke dodatka izravno (ne ugniježdene u podmapu).

Na primjer:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalirajte putem Moodle administratora

Prijavite se kao administrator stranice i navigirajte do **Site Administration > Notifications**. Moodle će otkriti novi dodatak i zatražiti da pokrenete instalaciju.

#### Konfigurirajte dodatak

Nakon instalacije, idite na **Site Administration > Plugins > Local plugins > FastComments** kako biste unijeli svoje postavke. Pogledajte odjeljak [Konfiguracija](#items-moodle-configuration) za detalje o svakoj opciji.