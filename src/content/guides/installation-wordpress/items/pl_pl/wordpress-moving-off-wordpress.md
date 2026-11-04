Jeśli przenosisz swoją witrynę z WordPress i chcesz używać FastComments na nowej stronie, nie potrzebujesz wtyczki WordPress. Wyeksportuj swoje komentarze z WordPress, a następnie prześlij plik na [stronie importu](https://fastcomments.com/auth/my-account/manage-data/import) w panelu FastComments.

Obsługujemy dwa formaty eksportu z WordPress.

### WordPress XML (zalecane)

To plik z wbudowanego eksportera WordPress, więc nie jest potrzebna dodatkowa wtyczka.

1. W panelu administracyjnym WordPress przejdź do `Tools -> Export`.
2. Wybierz `All content` i kliknij `Download Export File`.
3. Na [stronie importu](https://fastcomments.com/auth/my-account/manage-data/import) FastComments wybierz `WordPress (.xml)` i prześlij plik.

Każdy komentarz jest powiązany z adresem URL wpisu, na którym został dodany, który jest już zawarty w pliku.

Import zachowuje nazwę autora, adres e-mail i stronę internetową, datę, treść, wątki odpowiedzi oraz informację, czy komentarz został zatwierdzony. Awatary komentujących są przenoszone z Gravatara. Głosy nie są częścią tego formatu.

### WordPress CSV

To plik z [wtyczki WebToffee WordPress Comments Import & Export](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Zainstaluj wtyczkę w panelu administracyjnym WordPress i wyeksportuj swoje komentarze jako CSV.
2. Zastąp każdą wartość `comment_post_ID` adresem URL wpisu.
3. Na [stronie importu](https://fastcomments.com/auth/my-account/manage-data/import) FastComments wybierz `WordPress (.csv)` i prześlij plik.

Każdy komentarz jest powiązany z kolumną `comment_post_ID`. WordPress wypełnia tę kolumnę identyfikatorem wpisu, a Twoja nowa witryna nie ma identyfikatorów wpisów WordPress, dlatego w kroku 2 zastępujemy go adresem URL.

Import zachowuje nazwę autora, adres e-mail i stronę internetową, datę, treść, wątki odpowiedzi oraz informację, czy komentarz został zatwierdzony. Awatary komentujących są przenoszone z Gravatara. Zachowuje również flagę spamu WordPress oraz polubienia i polubienia w stylu "nie lubię" z wpDiscuz, gdy plik je zawiera.

### Dopasowywanie komentarzy do nowych stron

Jeśli Twoja nowa witryna zachowuje te same adresy URL co witryna WordPress, komentarze pojawią się na odpowiednich stronach bez dodatkowej konfiguracji.

Jeśli domena się zmienia, uruchom po imporcie [narzędzie migracji domen](/guide-migrations.html#migrating-domains). Jeśli zmieniają się adresy URL poszczególnych stron, możesz [zmigrować każdą stronę](/guide-migrations.html#migrating-pages) ze starego adresu URL na nowy.

W przypadku masowych migracji stron, na przykład usunięcia domeny z wartości przekazywanej do pola [urlId](/guide-customizations-and-configuration.html#url-id) widżetu komentarzy, [otwórz zgłoszenie do wsparcia](https://fastcomments.com/auth/my-account/help), a my zajmiemy się tym za Ciebie.

### Przed przełączeniem

Import możesz uruchamiać dowolną liczbę razy. Ponowny import tego samego pliku [nie tworzy duplikatów](/guide-migrations.html#importing-data), więc możesz zaimportować dane raz, aby przetestować nową witrynę, a następnie zaimportować je ponownie z najnowszymi komentarzami tuż przed przełączeniem.

W przypadku plików eksportu większych niż 1GB [skontaktuj się z pomocą techniczną](https://fastcomments.com/auth/my-account/help).

Aby dodać FastComments do swojej nowej witryny, zobacz [przewodnik instalacji](/guide-installation.html).