---
Nasza [wtyczka WordPress](https://wordpress.org/plugins/fastcomments/) ma potężny mechanizm importu oparty na interfejsie użytkownika. Po zainstalowaniu wtyczki przeprowadzi cię przez proces łączenia twojej instalacji WordPress z FastComments i skopiowania istniejących danych komentarzy.

**To odbywa się bez ręcznego kopiowania lub pobierania czegokolwiek.**

Przebieg migracji będzie wyświetlany w interfejsie użytkownika. Większość migracji trwa tylko kilka minut.

Mechanizm został zaprojektowany tak, aby nie obciążać nadmiernie twojej instalacji WordPress podczas migracji.

### CloudFlare & Zapory

Aby automatyczna konfiguracja WordPress działała, musimy wykonywać połączenia do twojej instalacji WordPress. Zapory sieciowe, takie jak Cloudflare, mogą nas zablokować i spowodować niepowodzenie integracji. W takich przypadkach [możemy dostarczyć ci](https://fastcomments.com/auth/my-account/help) zestaw adresów IP do dodania do białej listy dla tej integracji.

### Własność danych

W przypadku naszej migracji WordPress wszystkie nowe lub zaktualizowane dane komentarzy są automatycznie synchronizowane z twoją instalacją WordPress w tle. Oznacza to, że chociaż komentarze są dostarczane przez FastComments, aby odciążyć twoje środowisko WordPress, my **również** zapisujemy je w twojej bazie danych jako kopię zapasową. Oznacza to także, że jeśli zechcesz przejść z FastComments, twoje dane są już zmigrowane i aktualne.

---