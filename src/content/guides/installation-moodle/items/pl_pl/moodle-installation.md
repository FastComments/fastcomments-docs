#### Pobierz wtyczkę

Pobierz najnowszy plik ZIP wydania z <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">repozytorium FastComments Moodle na GitHub</a>.

#### Rozpakuj do katalogu instalacji Moodle

Rozpakuj plik ZIP w instalacji Moodle, tak aby wtyczka znajdowała się w `<moodle-root>/local/fastcomments`. Katalog wtyczki powinien zawierać bezpośrednio `version.php`, `lib.php` i inne pliki wtyczki (nie zagnieżdżone w podfolderze).

Na przykład:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Zainstaluj przez administratora Moodle

Zaloguj się jako administrator witryny i przejdź do **Administracja witryny > Powiadomienia**. Moodle wykryje nową wtyczkę i poprosi o uruchomienie instalacji.

#### Skonfiguruj wtyczkę

Po instalacji przejdź do **Administracja witryny > Wtyczki > Wtyczki lokalne > FastComments**, aby wprowadzić ustawienia. Zobacz sekcję [Konfiguracja](#items-moodle-configuration) po szczegóły dotyczące każdej opcji.