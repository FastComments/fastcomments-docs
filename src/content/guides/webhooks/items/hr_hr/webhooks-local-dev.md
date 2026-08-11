---
Za lokalni razvoj, koristite alat poput [ngrok](https://ngrok.com/).

Kako bi se pojednostavilo održavanje sigurnosti sustava, lokalni razvoj slijedi isti proces kao postavljanje i osiguravanje drugih okruženja. 

### Korak 1: Dodajte "localhost" u domene na vašem računu.

Dodajte "localhost" [kao domenu ovdje](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Obrazac za dodavanje domene u postavkama računa s unesenim localhostom u polje naziva domena'; title='Dodaj localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Odaberite API ključ

Dodavat ćemo konfiguraciju webhooka za vašu domenu, pa nam je potreban API ključ. [To možete učiniti ovdje.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Novi obrazac za API tajnu s pridruženom domenom postavljenom na localhost i ključem nazvanim Testing'; title='Dodaj API ključ za testiranje'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Pod "Associate with domain" – odaberite vašu domenu "localhost".

**NAPOMENA: Alternativno, možete koristiti jedan API tajni za sve testne aktivnosti i testna okruženja. Jednostavno dodajte API tajnu za "All Domains", i dajte joj ime poput "test".**

Provjerite imate li definiranu API tajnu za svoje produkcijske domene. Događaji za sve ostale domene koristit će wildcard (testnu) tajnu.

### Korak 3: Dodajte svoj webhook

Dok koristite ngrok ili sličan alat, postavite vrijednost za "localhost" [ovdje](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Administracija webhookova s odabranom domenom localhost i ngrok URL-om unesenim u krajnju točku za kreiranje komentara'; title='Dodaj testni webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Kad kliknete `Send Test Payload`, poslati ćemo dva testna događaja kako bismo provjerili da li validirate API ključ.

Kad se validira, pritisnite `Save`.

### Korak 4: Dodajte komentar

Sada možete dodavati, uređivati ili brisati komentare i trebali biste vidjeti kako pozivamo vaše lokalno razvojno računalo s događajima, koristeći vaš testni API ključ. Može proći do 30 sekundi prije nego što događaji stignu do vašeg računala.