Do lokalnego developmentu użyj narzędzia takiego jak [ngrok](https://ngrok.com/).

Aby uprościć utrzymywanie bezpieczeństwa systemu, lokalny development przebiega według tego samego procesu co konfiguracja i zabezpieczanie innych środowisk.

### Krok 1: Dodaj "localhost" do domen w swoim koncie.

Dodaj "localhost" [jako domenę tutaj](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Formularz dodawania domeny w ustawieniach konta z wpisanym localhost w polu nazw domen'; title='Dodaj localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Krok 2: Wybierz klucz API

Będziemy dodawać konfigurację webhooka dla Twojej domeny, więc będziemy potrzebować klucza API. [Możesz to zrobić tutaj.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Formularz nowego sekretu API z powiązaną domeną ustawioną na localhost i kluczem o nazwie Testing'; title='Dodaj testowy klucz API'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

W sekcji "Associate with domain" - wybierz swoją domenę "localhost".

**UWAGA: Alternatywnie możesz użyć jednego sekretu API dla całej aktywności testowej i środowisk staging. Po prostu dodaj sekret API dla "All Domains" i nadaj mu nazwę taką jak "test".**

Upewnij się, że masz zdefiniowany sekret API dla swojej domeny produkcyjnej (lub domen). Zdarzenia dla wszystkich innych domen będą używać sekretu wildcard (testowego).

### Krok 3: Dodaj swój webhook

Podczas działania ngrok lub podobnego narzędzia, ustaw wartość dla "localhost" [tutaj](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Panel administracyjny webhooków z wybraną domeną localhost i wypełnionym adresem URL ngrok w punkcie końcowym utworzenia komentarza'; title='Dodaj testowy webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Po kliknięciu `Send Test Payload` wyślemy dwa zdarzenia testowe, aby sprawdzić, czy walidujesz klucz API.

Po pomyślnej walidacji kliknij `Save`.

### Krok 4: Dodaj komentarz

Teraz możesz dodawać, edytować lub usuwać komentarze i powinieneś zobaczyć, jak wywołujemy Twoją lokalną maszynę deweloperską ze zdarzeniami, używając Twojego testowego klucza API. Może wystąpić opóźnienie do 30 sekund
zanim zdarzenia dotrą do Twojej maszyny.