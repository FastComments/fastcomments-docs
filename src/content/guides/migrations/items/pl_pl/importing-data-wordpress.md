Nasza [wtyczka WordPress](https://wordpress.org/plugins/fastcomments/) posiada potężny mechanizm importu oparty na interfejsie użytkownika. Po zainstalowaniu wtyczki
przeprowadzi Cię ona przez proces połączenia Twojej instalacji WordPress z FastComments oraz skopiowania istniejących danych komentarzy.

**Nie wymaga to ręcznego kopiowania ani pobierania czegokolwiek.**

Proces migracji będzie wskazywany w interfejsie użytkownika podczas migracji. Większość migracji zajmuje tylko kilka minut.

Mechanizm został zaprojektowany tak, aby nie powodować nadmiernego obciążenia Twojej instalacji WordPress podczas migracji.

Jeśli przenosisz swoją witrynę z WordPress, możesz zaimportować eksport WordPress w formacie XML lub CSV zamiast korzystać z wtyczki. Zobacz
[Przenoszenie komentarzy do nowej witryny](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare i zapory sieciowe

Aby automatyczna konfiguracja WordPress działała, musimy wykonywać wywołania do Twojej instalacji WordPress.
Zapory sieciowe, takie jak Cloudflare, mogą nas zablokować i spowodować niepowodzenie integracji. W takich przypadkach [możemy przekazać Ci](https://fastcomments.com/auth/my-account/help) zestaw adresów IP do dodania na białą listę dla integracji.

### Własność danych

W przypadku migracji z WordPress wszystkie nowe lub zaktualizowane dane komentarzy są automatycznie synchronizowane z powrotem do Twojej instalacji WordPress
w tle. Oznacza to, że chociaż komentarze są serwowane przez samo FastComments, aby odciążyć Twoją instalację WordPress,
my **również** zapisujemy je w Twojej bazie danych jako kopię zapasową. Oznacza to również, że jeśli zechcesz przejść z FastComments na inne rozwiązanie, Twoje dane
są już zmigrowane i aktualne.