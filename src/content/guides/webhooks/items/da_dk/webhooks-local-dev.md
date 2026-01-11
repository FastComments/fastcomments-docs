Til lokal udvikling, brug et værktøj som [ngrok](https://ngrok.com/).

For at gøre det nemmere at holde systemet sikkert, følger lokal udvikling den samme proces som opsætning og sikring af andre miljøer. 

### Trin 1: Tilføj "localhost" til domæner på din konto.

Tilføj "localhost" [som et domæne her](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Trin 2: Vælg en API Key

Vi skal tilføje webhook-konfiguration for dit domæne, så vi får brug for en API-nøgle. [Det kan du gøre her.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - vælg dit "localhost" domæne.

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Sørg for, at du har en API Secret defineret for dine produktionsdomæner. Begivenheder for alle andre domæner vil bruge wildcard (testing) secret.

### Trin 3: Tilføj din webhook

Mens ngrok eller et lignende værktøj kører, sæt værdien for "localhost" [her](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Trin 4: Tilføj en kommentar

Nu kan du tilføje, redigere eller slette kommentarer og bør se, at vi ringer til din lokale udviklingsmaskine med begivenhederne ved hjælp af din test-API-nøgle. Der kan være op til 30 sekunders forsinkelse
for begivenhederne at nå din maskine.