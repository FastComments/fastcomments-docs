Za lokalni razvoj uporabite orodje, kot je [ngrok](https://ngrok.com/).

Da bi poenostavili ohranjanje varnosti sistema, lokalni razvoj sledi istemu postopku kot nastavitev in zavarovanje drugih okolij. 

### Korak 1: Dodajte "localhost" med domene v vašem računu.

Dodajte "localhost" [kot domeno tukaj](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Izberite API ključ

Dodajali bomo konfiguracijo webhooka za vašo domeno, zato potrebujemo API ključ. [To lahko uredite tukaj.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Pod "Associate with domain" - izberite vašo domeno "localhost".

**OPOMBA: Alternativno lahko uporabite en sam API Secret za vse testne aktivnosti in predprodukcijska okolja. Preprosto dodajte API Secret za "All Domains", in mu dajte ime, na primer "test".**

Poskrbite, da imate za vaše produkcijske domene definirano API Secret. Dogodki za vse ostale domene bodo uporabljali wildcard (testni) secret.

### Korak 3: Dodajte vaš webhook

Med uporabo ngrok-a ali podobnega orodja nastavite vrednost za "localhost" [tukaj](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Korak 4: Dodajte komentar

Zdaj lahko dodate, uredite ali izbrišete komentarje in bi morali videti, da sistem kliče vaš lokalni razvojni računalnik z dogodki, pri čemer uporablja vaš testni API ključ. Morda bo do 30 sekund zamude
prejema dogodkov na vaš računalnik.

---