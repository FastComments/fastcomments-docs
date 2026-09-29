Nasza [Wtyczka WordPress](https://wordpress.org/plugins/fastcomments/) posiada potężny mechanizm importu oparty na interfejsie użytkownika. Po zainstalowaniu wtyczki poprowadzi Cię przez połączenie Twojej instalacji WordPress z FastComments oraz skopiowanie istniejących danych komentarzy.

**Odbywa się to bez ręcznego kopiowania lub pobierania czegokolwiek.**

Proces migracji będzie wskazywany w interfejsie użytkownika podczas migracji. Większość migracji zajmuje tylko kilka minut.

Mechanizm został zaprojektowany tak, aby nie obciążać nadmiernie Twojej instalacji WordPress podczas migracji.

Jeśli przenosisz swoją witrynę z WordPressa, możesz zaimportować eksport WordPress w formacie XML lub CSV zamiast używać wtyczki. Zobacz [Przenoszenie komentarzy na nową witrynę](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare i zapory sieciowe

Aby automatyczna konfiguracja WordPress działała, musimy wykonywać wywołania do Twojej instalacji WordPress. Zapory, takie jak Cloudflare, mogą nas zablokować i spowodować niepowodzenie integracji. W takich przypadkach [możemy Ci zapewnić](https://fastcomments.com/auth/my-account/help) zestaw adresów IP do wpisania na białą listę dla integracji.

### Własność danych

W przypadku naszej migracji WordPress, wszystkie nowe lub zaktualizowane dane komentarzy są automatycznie synchronizowane z powrotem do Twojej instalacji WordPress w tle. Oznacza to, że choć komentarze są serwowane bezpośrednio przez FastComments, aby odciążyć Twoją instalację WordPress, **również** zapisujemy je w Twojej bazie danych jako kopię zapasową. To także oznacza, że jeśli zechcesz przejść z FastComments, Twoje dane są już migrowane i aktualne.