Til lokal udvikling, brug et værktøj som [ngrok](https://ngrok.com/).

For at gøre det nemmere at holde systemet sikkert følger lokal udvikling den samme proces som opsætning og sikring af andre miljøer. 

### Trin 1: Tilføj "localhost" til domæner i din konto.

Tilføj "localhost" [som et domæne her](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Trin 2: Pick an API Key

Vi vil tilføje webhook-konfiguration for dit domæne, så vi får brug for en API Key. [Du kan gøre det her.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - vælg dit "localhost" domæne.

**NOTE: Alternativt kan du bruge én API Secret til al testaktivitet og staging-miljøer. Tilføj simpelthen en API Secret for "All Domains", og giv den et navn som "test".**

Sørg for, at du har en API Secret defineret for dine produktionsdomæner. Events for alle andre domæner vil bruge wildcard (testing) secret.

### Trin 3: Add Your Webhook

Mens ngrok eller et lignende værktøj kører, sæt værdien for "localhost" [her](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Når du klikker på `Send Test Payload`, sender vi to test events for at kontrollere, at du validerer API key'en.

Når den er valideret, klik på `Save`.

### Trin 4: Add A Comment

Nu kan du tilføje, redigere eller slette kommentarer og burde se, at vi kalder din lokale udviklingsmaskine med events, ved brug af din testing API Key. Der kan være op til 30 sekunders forsinkelse
for events at nå din maskine.