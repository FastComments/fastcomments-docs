#### Prenesite vtičnik

Prenesite najnovejšo izdajo v obliki ZIP iz <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repozitorija</a>.

#### Razpakirajte v imenik Moodla

Razpakirajte ZIP v vašo namestitev Moodla tako, da bo vtičnik lociran v `<moodle-root>/local/fastcomments`. Mapa vtičnika naj neposredno vsebuje `version.php`, `lib.php` in druge datoteke vtičnika (ne v podmapi).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Namestitev prek skrbnika Moodla

Prijavite se kot skrbnik strani in pojdite na **Administracija spletnega mesta > Obvestila**. Moodle bo zaznal nov vtičnik in vas pozval, da zaženete namestitev.

#### Konfigurirajte vtičnik

Po namestitvi pojdite na **Administracija spletnega mesta > Vtičniki > Lokalni vtičniki > FastComments**, kjer vnesete svoje nastavitve. Oglejte si razdelek [Konfiguracija](#moodle-configuration) za podrobnosti o vsaki možnosti.

---