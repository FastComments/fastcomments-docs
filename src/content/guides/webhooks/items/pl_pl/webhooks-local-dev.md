Do lokalnego środowiska deweloperskiego użyj narzędzia takiego jak [ngrok](https://ngrok.com/).

Aby uprościć utrzymanie bezpieczeństwa systemu, środowisko lokalne korzysta z tego samego procesu konfiguracji i zabezpieczania co inne środowiska. 

### Step 1: Add "localhost" to domains in your account.

Dodaj "localhost" [jako domenę tutaj](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: Pick an API Key

Dodamy konfigurację webhooka dla Twojej domeny, więc potrzebujemy klucza API. [Możesz to zrobić tutaj.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

W sekcji "Associate with domain" – wybierz swoją domenę "localhost".

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Upewnij się, że masz zdefiniowany API Secret dla swojej domeny produkcyjnej(ych). Zdarzenia dla wszystkich innych domen będą używać wildcard (testowego) sekretu.

### Step 3: Add Your Webhook

Podczas uruchamiania ngrok lub podobnego narzędzia ustaw wartość dla "localhost" [tutaj](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

Po kliknięciu `Send Test Payload` wyślemy dwa zdarzenia testowe, aby sprawdzić, czy poprawnie weryfikujesz klucz API.

Gdy zostanie zweryfikowany, kliknij `Save`.

### Step 4: Add A Comment

Teraz możesz dodawać, edytować lub usuwać komentarze i powinieneś zobaczyć, że będziemy wywoływać Twoją lokalną maszynę deweloperską z tymi zdarzeniami, używając testowego klucza API. Może wystąpić opóźnienie do 30 sekund, zanim zdarzenia dotrą do Twojej maszyny.

---