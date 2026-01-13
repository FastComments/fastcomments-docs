Za lokalni razvoj koristite alat poput [ngrok](https://ngrok.com/).

Kako bismo pojednostavili održavanje sigurnosti sustava, lokalni razvoj slijedi isti postupak kao i postavljanje i osiguravanje drugih okruženja. 

### Korak 1: Dodajte "localhost" među domene na vašem računu.

Dodajte "localhost" [kao domenu ovdje](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Odaberite API ključ

Dodavat ćemo konfiguraciju web-hooka za vašu domenu, pa ćemo trebati API ključ. [To možete napraviti ovdje.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NAPOMENA: Alternativno, možete koristiti jedan API Secret za sve testne aktivnosti i staging okruženja. Jednostavno dodajte API Secret za "All Domains" i dajte mu ime poput "test".**

Pobrinite se da imate definirani API Secret za svoje produkcijske domene. Događaji za sve ostale domene koristit će wildcard (testing) secret.

### Korak 3: Dodajte svoj webhook

Dok pokrećete ngrok ili sličan alat, postavite vrijednost za "localhost" [ovdje](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Klikom na `Send Test Payload` poslat ćemo dva testna događaja kako bismo provjerili valjanost vašeg API ključa.

Kad se potvrdi, pritisnite `Save`.

### Korak 4: Dodajte komentar

Sada možete dodavati, uređivati ili brisati komentare i trebali biste vidjeti kako pozivamo vaše lokalno razvojno računalo s događajima, koristeći vaš testni API ključ. Može doći do kašnjenja do 30 sekundi
za događaji stignu do vašeg računala.