For Local development, use a tool like [ngrok](https://ngrok.com/).

In order to simplify keeping the system secure, local development follows the same process as setting up and securing other environments. 

### Korak 1: Dodajte "localhost" u domene na svom nalogu.

Dodajte "localhost" [kao domen ovde](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Korak 2: Odaberite API Key

Dodaćemo konfiguraciju webhook-a za vaš domen, pa će nam trebati API Key. [To možete uraditi ovde.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NAPOMENA: Alternativno, možete koristiti jedan API Secret za svu test aktivnost i staging okruženja. Jednostavno dodajte API Secret za "All Domains", i nazovite ga npr. "test".**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### Korak 3: Dodajte svoj webhook

Dok pokrećete ngrok ili sličan alat, podesite vrednost za "localhost" [ovde](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Korak 4: Dodajte komentar

Sada možete dodavati, izmenjivati ili brisati komentare i trebalo bi da vidite kako pozivamo vašu lokalnu razvojnu mašinu sa događajima, koristeći vaš testing API key. Može biti do 30 sekundi kašnjenja dok događaji ne stignu do vaše mašine.

---