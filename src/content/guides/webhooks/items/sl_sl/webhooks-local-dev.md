Za lokalni razvoj uporabite orodje, kot je [ngrok](https://ngrok.com/).

Da bi poenostavili ohranjanje varnosti sistema, lokalni razvoj sledi istemu postopku kot nastavitev in zavarovanje drugih okolij. 

### Korak 1: Dodajte "localhost" med domene v vašem računu.

Dodajte "localhost" [kot domeno tukaj](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Dodaj localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Izberite API ključ

Dodali bomo konfiguracijo webhooka za vašo domeno, zato bomo potrebovali API ključ. [To lahko naredite tukaj.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Dodaj testni API ključ'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Poskrbite, da imate za svoje produkcijske domene definirano API skrivnost. Dogodki za vse ostale domene bodo uporabili nadomestno (testno) skrivnost.

### Korak 3: Dodajte svoj webhook

Med izvajanjem ngrok ali podobnega orodja nastavite vrednost za "localhost" [tukaj](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Dodaj testni webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Korak 4: Dodajte komentar

Zdaj lahko dodajate, urejate ali brišete komentarje in morali bi videti, da bo sistem klical vašo lokalno razvojno napravo z dogodki, pri čemer bo uporabljen vaš testni API ključ. Lahko pride do zamude do 30 sekund za prihod dogodkov do vaše naprave.

---