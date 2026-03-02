#### Preuzmite dodatak

Preuzmite najnoviji ZIP iz <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repozitorijuma</a>.

#### Raspakujte u vaš Moodle direktorijum

Raspakujte ZIP u vašu Moodle instalaciju tako da dodatak bude smešten u `<moodle-root>/local/fastcomments`. Direktorijum dodatka treba da sadrži `version.php`, `lib.php`, i ostale fajlove dodatka direktno (ne u poddirektorijumu).

Na primer:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalirajte preko Moodle administratora

Prijavite se kao administrator sajta i idite na **Administracija sajta > Obaveštenja**. Moodle će otkriti novi dodatak i zatražiti od vas da pokrenete instalaciju.

#### Podesite dodatak

Nakon instalacije, idite na **Administracija sajta > Dodaci > Lokalni dodaci > FastComments** da unesete vaše podešavanja. Pogledajte [Konfiguracija](#moodle-configuration) odeljak za detalje o svakoj opciji.