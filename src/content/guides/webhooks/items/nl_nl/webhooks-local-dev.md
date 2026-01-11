Voor lokale ontwikkeling, gebruik een tool zoals [ngrok](https://ngrok.com/).

Om het eenvoudiger te maken om het systeem veilig te houden, volgt lokale ontwikkeling hetzelfde proces als het opzetten en beveiligen van andere omgevingen. 

### Stap 1: Voeg "localhost" toe aan de domeinen in uw account.

Voeg "localhost" [als een domein hier](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Stap 2: Kies een API Key

We gaan webhook-configuratie toevoegen voor uw domein, dus we hebben een API key nodig. [Dat kunt u hier doen.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Onder "Associate with domain" - selecteer uw "localhost" domein.

**OPMERKING: Als alternatief kunt u één API Secret gebruiken voor alle testactiviteiten en staging-omgevingen. Voeg simpelweg een API Secret toe voor "All Domains", en geef het een naam zoals "test".**

Zorg dat u een API Secret heeft gedefinieerd voor uw productie-domein(en). Gebeurtenissen voor alle andere domeinen gebruiken de wildcard (testing) secret.

### Stap 3: Voeg uw webhook toe

Terwijl u ngrok of een vergelijkbare tool draait, stelt u de waarde voor "localhost" [hier in](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Stap 4: Voeg een reactie toe

Nu kunt u reacties toevoegen, bewerken of verwijderen en zou u moeten zien dat wij uw lokale ontwikkelmachine aanroepen met de gebeurtenissen, waarbij uw test-API-sleutel wordt gebruikt. Er kan een vertraging van maximaal 30 seconden zijn
voordat de gebeurtenissen uw machine bereiken.

---