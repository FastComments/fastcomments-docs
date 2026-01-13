Dla lokalnego rozwoju użyj narzędzia takiego jak [ngrok](https://ngrok.com/).

Aby uprościć utrzymanie bezpieczeństwa systemu, lokalny rozwój stosuje taki sam proces jak konfiguracja i zabezpieczanie innych środowisk. 

### Krok 1: Dodaj "localhost" do domen w swoim koncie.

Dodaj "localhost" [jako domenę tutaj](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Krok 2: Wybierz klucz API

Będziemy dodawać konfigurację webhooka dla Twojej domeny, więc potrzebny będzie klucz API. [Możesz to zrobić tutaj.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Pod "Powiąż z domeną" - wybierz swoją domenę "localhost".

**UWAGA: Alternatywnie możesz użyć jednego sekretu API dla całej aktywności testowej i środowisk staging. Po prostu dodaj sekret API dla "Wszystkie domeny", i nadaj mu nazwę, taką jak "test".**

Upewnij się, że masz zdefiniowany sekret API dla swoich domen produkcyjnych. Zdarzenia dla wszystkich pozostałych domen będą korzystać z sekretu wildcard (testowego).

### Krok 3: Dodaj swój webhook

Podczas uruchomienia ngrok lub podobnego narzędzia ustaw wartość dla "localhost" [tutaj](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Po kliknięciu `Send Test Payload` wyślemy dwa testowe zdarzenia, aby sprawdzić, czy weryfikujesz klucz API.

Gdy zostanie zweryfikowany, kliknij `Save`.

### Krok 4: Dodaj komentarz

Teraz możesz dodawać, edytować lub usuwać komentarze i powinieneś zobaczyć, że będziemy wywoływać Twoją maszynę deweloperską lokalnie z tymi zdarzeniami, używając testowego klucza API. Może wystąpić opóźnienie do 30 sekund
zanim zdarzenia dotrą do Twojej maszyny.