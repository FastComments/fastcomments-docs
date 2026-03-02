#### Prenesite vtičnik

Prenesite ZIP najnovejše izdaje iz <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">GitHub repozitorij FastComments Moodle</a>.

#### Razpakirajte v svojo namestitev Moodle

Razpakirajte ZIP v vašo Moodle namestitev tako, da bo vtičnik nameščen na `<moodle-root>/local/fastcomments`. Mapa vtičnika naj neposredno vsebuje `version.php`, `lib.php` in druge datoteke vtičnika (ne v podmapi).

Na primer:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Namestite prek Moodle skrbnika

Prijavite se kot skrbnik spletnega mesta in pojdite v **Upravljanje mesta > Obvestila**. Moodle bo zaznal nov vtičnik in vas pozval, da zaženete namestitev.

#### Konfigurirajte vtičnik

Po namestitvi pojdite na **Upravljanje mesta > Vtičniki > Lokalni vtičniki > FastComments**, da vnesete nastavitve. Oglejte si razdelek [Konfiguracija](#moodle-configuration) za podrobnosti o vsaki možnosti.