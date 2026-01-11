Za lokalni razvoj, koristite alat poput [ngrok](https://ngrok.com/).

Kako bismo pojednostavili održavanje sigurnosti sustava, lokalni razvoj slijedi isti postupak kao i postavljanje i osiguravanje drugih okruženja. 

### Korak 1: Dodajte "localhost" među domene u svom računu.

Dodajte "localhost" [kao domenu ovdje](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Odaberite API ključ

Dodavat ćemo konfiguraciju webhooka za vašu domenu, pa ćemo trebati API ključ. [Možete to učiniti ovdje.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Pod "Associate with domain" - odaberite svoju domenu "localhost".

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Pobrinite se da imate definirani API Secret za vaše produkcijske domene. Događaji za sve ostale domene koristit će wildcard (testing) secret.

### Korak 3: Dodajte svoj webhook

Dok pokrećete ngrok ili sličan alat, postavite vrijednost za "localhost" [ovdje](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Korak 4: Dodajte komentar

Sada možete dodavati, uređivati ili brisati komentare i trebali biste vidjeti kako pozivamo vašu lokalnu razvojnu mašinu s događajima, koristeći svoj testni API ključ. Može doći do kašnjenja do 30 sekundi
prije nego što događaji stignu do vaše mašine.