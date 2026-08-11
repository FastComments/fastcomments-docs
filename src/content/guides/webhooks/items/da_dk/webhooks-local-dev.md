For lokal udvikling kan du bruge et værktøj som [ngrok](https://ngrok.com/).

For at gøre det lettere at holde systemet sikkert, følger lokal udvikling den samme proces som opsætning og sikring af andre miljøer. 

### Trin 1: Tilføj "localhost" til domæner i din konto.

Tilføj "localhost" [som et domæne her](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Formularen til at tilføje domæne i kontoindstillingerne med localhost indtastet i feltet for domænenavne'; title='Tilføj localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Trin 2: Vælg en API-nøgle

Vi skal tilføje webhook-konfiguration for dit domæne, så vi har brug for en API-nøgle. [Du kan gøre det her.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Ny API-hemmelighedsformular med det tilknyttede domæne sat til localhost og nøglen navngivet Testing'; title='Tilføj test-API-nøgle'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under “Associate with domain” - vælg dit “localhost”-domæne.

**BEMÆRK: Alternativt kan du bruge én API-hemmelighed til al testaktivitet og staging-miljøer. Tilføj blot en API-hemmelighed for “All Domains”, og giv den et navn som “test”.**

Sørg for, at du har en API-hemmelighed defineret for dine produktionsdomæner. Begivenheder for alle andre domæner vil bruge wildcard‑ (test‑) hemmeligheden.

### Trin 3: Tilføj din webhook

Mens du kører ngrok eller et lignende værktøj, indstil værdien for “localhost” [her](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Webhook‑admin med localhost‑domænet valgt og en ngrok‑URL indtastet i endpointet for oprettet kommentar'; title='Tilføj test‑webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Når du klikker på `Send Test Payload`, vil vi sende to test‑begivenheder for at kontrollere, at du validerer API‑nøglen.

Når den er valideret, tryk på `Save`.

### Trin 4: Tilføj en kommentar

Nu kan du tilføje, redigere eller slette kommentarer og bør se, at vi kalder din lokale udviklingsmaskine med begivenhederne ved hjælp af din test‑API‑nøgle. Der kan gå op til 30 sekunder, før begivenhederne når din maskine.

---