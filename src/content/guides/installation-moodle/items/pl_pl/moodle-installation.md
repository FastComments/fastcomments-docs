#### Pobierz wtyczkę

Pobierz najnowsze wydanie w formacie ZIP z <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">repozytorium FastComments Moodle na GitHubie</a>.

#### Wyodrębnij do katalogu Moodle

Wypakuj plik ZIP do instalacji Moodle tak, aby wtyczka znajdowała się w `<moodle-root>/local/fastcomments`. Katalog wtyczki powinien zawierać `version.php`, `lib.php` oraz inne pliki wtyczki bezpośrednio (nie zagnieżdżone w podfolderze).

Na przykład:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Zainstaluj przez panel administracyjny Moodle

Zaloguj się jako administrator witryny i przejdź do **Administracja witryny > Powiadomienia**. Moodle wykryje nową wtyczkę i poprosi o uruchomienie instalacji.

#### Skonfiguruj wtyczkę

Po instalacji przejdź do **Administracja witryny > Wtyczki > Wtyczki lokalne > FastComments**, aby wprowadzić ustawienia. Zobacz sekcję [Konfiguracja](#moodle-configuration) po szczegóły dotyczące każdej opcji.