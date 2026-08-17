Nasz [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ma potężny mechanizm importu oparty na interfejsie użytkownika. Po zainstalowaniu wtyczki,
poprowadzi Cię przez połączenie Twojej instalacji WordPress z FastComments i skopiowanie istniejących danych komentarzy.

**Dzieje się to bez ręcznego kopiowania lub pobierania czegokolwiek.**

Proces migracji będzie wskazywany w interfejsie użytkownika podczas migracji. Większość migracji zajmuje tylko kilka minut.

Mechanizm został zaprojektowany tak, aby nie obciążać nadmiernie Twojej instalacji WordPress podczas migracji.

### CloudFlare & FireWalls

Aby automatyczna konfiguracja WordPress działała, musimy wykonywać wywołania do Twojej instalacji WordPress.
Zapory takie jak Cloudflare mogą nas zablokować i spowodować niepowodzenie integracji. W takich przypadkach [możemy
Ci](https://fastcomments.com/auth/my-account/help) dostarczyć zestaw adresów IP do wpisania na białą listę dla integracji.

### Data Ownership

W przypadku naszej migracji WordPress, wszelkie nowe lub zaktualizowane dane komentarzy są automatycznie synchronizowane z powrotem do Twojej instalacji WordPress
w tle. Oznacza to, że podczas gdy komentarze są serwowane przez FastComments, aby odciążyć Twoją instalację WordPress,
**również** zapisujemy je w Twojej bazie danych jako kopię zapasową. To także oznacza, że jeśli zechcesz przejść z FastComments, Twoje dane są
już migrowane i aktualne.