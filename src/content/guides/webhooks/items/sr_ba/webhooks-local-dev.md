Za lokalni razvoj, koristite alat poput [ngrok](https://ngrok.com/).

Da biste pojednostavili održavanje sigurnosti sistema, lokalni razvoj prati isti postupak kao i postavljanje i osiguravanje drugih okruženja. 

### Korak 1: Dodajte "localhost" u domene na vašem nalogu.

Dodajte "localhost" [kao domen ovdje](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Odaberite API ključ

Dodavat ćemo konfiguraciju webhook-a za vaš domen, pa će nam trebati API ključ. [Možete to uraditi ovdje.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

U polju "Associate with domain" - odaberite vaš "localhost" domen.

**NAPOMENA: Alternativno, možete koristiti jedan API Secret za svu test aktivnost i staging okruženja. Jednostavno dodajte API Secret za "All Domains", i dajte mu ime poput "test".**

Osigurajte da imate definisan API Secret za vaše produkcijske domene. Događaji za sve ostale domene će koristiti wildcard (testni) secret.

### Korak 3: Dodajte svoj webhook

Dok pokrećete ngrok ili sličan alat, postavite vrijednost za "localhost" [ovdje](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Korak 4: Dodajte komentar

Sada možete dodavati, uređivati ili brisati komentare i trebali biste vidjeti da pozivamo vašu lokalnu mašinu za razvoj sa događajima, koristeći vaš testni API ključ. Može postojati do 30 sekundi kašnjenja
prije nego što događaji stignu do vaše mašine.