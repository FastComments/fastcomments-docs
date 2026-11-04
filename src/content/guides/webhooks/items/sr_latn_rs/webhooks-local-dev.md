Za lokalni razvoj koristite alat kao što je [ngrok](https://ngrok.com/).

Kako bi održavanje sistema bezbednim bilo jednostavnije, lokalni razvoj prati isti proces kao podešavanje i obezbeđivanje ostalih okruženja.

### Korak 1: Dodajte "localhost" u domene svog naloga.

Dodajte "localhost" [kao domen ovde](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Forma za dodavanje domena u podešavanjima naloga sa unetim localhost u polju za imena domena'; title='Dodaj localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Izaberite API ključ

Dodaćemo konfiguraciju webhook-a za vaš domen, pa će nam trebati API ključ. [To možete uraditi ovde.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Forma za novi API secret sa povezanim domenom postavljenim na localhost i ključem nazvanim Testing'; title='Dodaj Testing API ključ'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Pod "Associate with domain" - izaberite svoj "localhost" domen.

**NAPOMENA: Alternativno, možete koristiti jedan API Secret za sve test aktivnosti i staging okruženja. Jednostavno dodajte API Secret za "All Domains" i dajte mu ime kao što je "test".**

Proverite da imate definisan API Secret za vaše produkcijske domene. Događaji za sve ostale domene koristiće wildcard (testni) secret.

### Korak 3: Dodajte svoj Webhook

Dok ngrok ili sličan alat radi, postavite vrednost za "localhost" [ovde](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Webhooks administracija sa izabranim localhost domenom i ngrok URL-om upisanim u endpoint za kreirani komentar'; title='Dodaj testni webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Kada kliknete `Send Test Payload`, poslaćemo dva testna događaja da proverimo da validirate API ključ.

Kada validacija prođe, kliknite `Save`.

### Korak 4: Dodajte komentar

Sada možete dodavati, menjati ili brisati komentare i trebalo bi da vidimo da pozivamo vašu lokalnu razvojnu mašinu sa događajima, koristeći vaš testni API ključ. Može postojati kašnjenje do 30 sekundi
dok događaji stignu do vaše mašine.