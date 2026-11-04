Za lokalni razvoj uporabite orodje, kot je [ngrok](https://ngrok.com/).

Da poenostavimo vzdrževanje varnosti sistema, lokalni razvoj sledi istemu postopku kot nastavitev in zavarovanje drugih okolij.

### Korak 1: Dodajte "localhost" med domene v vašem računu.

Dodajte "localhost" [kot domeno tukaj](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Obrazec za dodajanje domene v nastavitvah računa z vnesenim localhost v polju za imena domen'; title='Dodaj localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Izberite API ključ

Dodali bomo konfiguracijo webhooka za vašo domeno, zato potrebujemo API ključ. [To lahko storite tukaj.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Obrazec za nov API skrivni ključ s povezano domeno, nastavljeno na localhost, in ključem poimenovanim Testing'; title='Dodaj testni API ključ'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Pod "Associate with domain" - izberite svojo domeno "localhost".

**OPOMBA: Alternativno lahko uporabite en API skrivni ključ za vso testno dejavnost in testna (staging) okolja. Preprosto dodajte API skrivni ključ za "All Domains" in ga poimenujte npr. "test".**

Poskrbite, da imate definiran API skrivni ključ za vaše produkcijske domene. Dogodki za vse ostale domene bodo uporabili wildcard (testni) ključ.

### Korak 3: Dodajte svoj webhook

Medtem ko teče ngrok ali podobno orodje, nastavite vrednost za "localhost" [tukaj](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Skrbniški vmesnik webhookov z izbrano domeno localhost in ngrok URL-jem, vnesenim v endpoint za ustvarjen komentar'; title='Dodaj testni webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Ko kliknete `Send Test Payload`, bomo poslali dva testna dogodka, da preverimo, ali potrdujete API ključ.

Ko se potrditev uspešno zaključi, kliknite `Save`.

### Korak 4: Dodajte komentar

Zdaj lahko dodajate, urejate ali brišete komentarje in videli boste, da pokličemo vaš lokalni razvojni računalnik z dogodki, pri čemer uporabljamo vaš testni API ključ. Lahko pride do zamika do 30 sekund,
preden dogodki dosežejo vaš računalnik.