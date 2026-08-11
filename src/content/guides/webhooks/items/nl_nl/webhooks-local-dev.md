For lokale ontwikkeling, gebruik een tool zoals [ngrok](https://ngrok.com/).

Om het beveiligen van het systeem te vereenvoudigen, volgt lokale ontwikkeling hetzelfde proces als het opzetten en beveiligen van andere omgevingen. 

### Stap 1: Voeg "localhost" toe aan domeinen in uw account.

Voeg "localhost" [als een domein hier toe](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Het formulier om een domein toe te voegen in accountinstellingen met localhost ingevoerd in het veld domeinnamen'; title='Voeg localhost toe'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Stap 2: Kies een API-sleutel

We gaan webhookconfiguratie voor uw domein toevoegen, dus hebben we een API-sleutel nodig. [U kunt dat hier doen.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Nieuw API-secret formulier met het gekoppelde domein ingesteld op localhost en de sleutel genaamd Testing'; title='Voeg test-API-sleutel toe'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Onder "Associate with domain" - selecteer uw "localhost" domein.

**OPMERKING: Als alternatief kunt u één API-secret gebruiken voor alle testactiviteiten en staging-omgevingen. Voeg eenvoudig een API-secret toe voor "All Domains", en geef het een naam zoals "test".**

Zorg ervoor dat u een API-secret heeft gedefinieerd voor uw productiedomein(en). Gebeurtenissen voor alle andere domeinen zullen de wildcard (test) secret gebruiken.

### Stap 3: Voeg uw webhook toe

Terwijl u ngrok of een vergelijkbare tool draait, stel de waarde voor "localhost" [hier](https://fastcomments.com/auth/my-account/manage-data/webhooks) in.

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Webhooks-beheerder met het localhost-domein geselecteerd en een ngrok-URL ingevuld in het endpoint voor aangemaakte opmerkingen'; title='Voeg test-webhook toe'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Bij het klikken op `Send Test Payload` zullen we twee testgebeurtenissen verzenden om te controleren of u de API-sleutel valideert.

Zodra het gevalideerd is, klik op `Save`.

### Stap 4: Voeg een reactie toe

Nu kunt u reacties toevoegen, bewerken of verwijderen en zou moeten zien dat we uw lokale ontwikkelmachine aanroepen met de gebeurtenissen, met behulp van uw test-API-sleutel. Er kan een vertraging van maximaal 30 seconden zijn voordat de gebeurtenissen uw machine bereiken.

---