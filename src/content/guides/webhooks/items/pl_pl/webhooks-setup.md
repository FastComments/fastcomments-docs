Wykonaj te same kroki dla `localhost`, co dla środowiska produkcyjnego. Upewnij się, że masz skonfigurowane domeny produkcyjne i API Secrets.

Najpierw przejdź do [Administracja webhookami](https://fastcomments.com/auth/my-account/manage-data/webhooks). Jest to dostępne przez Zarządzaj danymi -> Webhooki.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

Na tej stronie możesz określić endpoints dla każdego typu zdarzenia komentarza.

Dla każdego typu zdarzenia kliknij Send Test Payload, aby upewnić się, że integracja została poprawnie skonfigurowana. Zobacz następną sekcję "Testowanie" po szczegóły.