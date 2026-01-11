Za lokalni razvoj, koristite alat kao što je [ngrok](https://ngrok.com/).

Da bismo pojednostavili održavanje bezbednosti sistema, lokalni razvoj prati isti proces kao i podešavanje i osiguravanje drugih okruženja. 

### Korak 1: Dodajte "localhost" u domene na svom nalogu.

Dodajte "localhost" [kao domen ovde](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Izaberite API ključ

Dodavaćemo konfiguraciju webhook-a za vaš domen, tako da će nam trebati API ključ. [To možete uraditi ovde.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Pod "Associate with domain" - izaberite vaš "localhost" domen.

**NAPOMENA: Alternativno, možete koristiti jedan API Secret za svu test aktivnost i staging okruženja. Jednostavno dodajte API Secret za "All Domains", i dajte mu ime poput "test".**

Uverite se da imate definisan API Secret za vaše produkcione domene. Događaji za sve ostale domene koristiće wildcard (testing) secret.

### Korak 3: Dodajte vaš webhook

Dok pokrećete ngrok ili sličan alat, podesite vrednost za "localhost" [ovde](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

Klikom na `Send Test Payload`, poslaćemo dva test događaja da proverimo da li validirate API ključ.

Kada se potvrdi, pritisnite `Save`.

### Korak 4: Dodajte komentar

Sada možete dodavati, menjati ili brisati komentare i trebalo bi da vidite pozive ka vašoj lokalnoj razvojnoj mašini sa događajima, koristeći vaš testing API key. Može postojati zakašnjenje do 30 sekundi
pre nego što događaji stignu do vaše mašine.