In de Webhooks-admin staan `Send Test Payload`-knoppen voor elk gebeurtenistype (Create, Update, Delete). De Create- en Update-gebeurtenissen sturen een dummy `WebhookComment`-object, terwijl bij het testen van Delete een dummy request body met alleen een ID wordt verzonden.

## Verifiëren van payloads

Controleer bij het testen van je webhook-integratie of de binnenkomende verzoeken de volgende headers bevatten:

1. **`token`** - Your API Secret
2. **`X-FastComments-Timestamp`** - Unix timestamp (seconds)
3. **`X-FastComments-Signature`** - HMAC-SHA256 signature

Gebruik de HMAC-handtekeningverificatie om te zorgen dat payloads authentiek zijn.

## Testtools

Je kunt tools zoals [webhook.site](https://webhook.site) of [ngrok](https://ngrok.com) gebruiken om binnenkomende webhook-payloads tijdens de ontwikkeling te inspecteren.

## Gebeurtenistypen

- **Create-gebeurtenis**: Wordt geactiveerd wanneer een nieuwe reactie wordt aangemaakt. Standaardmethode: PUT
- **Update-gebeurtenis**: Wordt geactiveerd wanneer een reactie wordt bewerkt. Standaardmethode: PUT
- **Delete-gebeurtenis**: Wordt geactiveerd wanneer een reactie wordt verwijderd. Standaardmethode: DELETE

Elke gebeurtenis bevat de volledige reactiegegevens in de request body (zie [Gegevensstructuren](/guides/webhooks/webhooks-structures) voor het payloadformaat).