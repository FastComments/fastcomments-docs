In de Webhooks-admin zijn `Send Test Payload`-knoppen voor elk evenementstype (Create, Update, Delete). De Create- en Update-events sturen een dummy `WebhookComment`-object, terwijl bij het testen van Delete een dummy verzoekbody wordt gestuurd met alleen een ID.

## Payloads verifiëren

When testing your webhook integration, verify the incoming requests include the following headers:

1. **`token`** - Uw API-secret
2. **`X-FastComments-Timestamp`** - Unix-tijdstempel (seconden)
3. **`X-FastComments-Signature`** - HMAC-SHA256-handtekening

Gebruik de HMAC-handtekeningverificatie om te controleren of de payloads authentiek zijn.

## Testtools

U kunt tools zoals [webhook.site](https://webhook.site) of [ngrok](https://ngrok.com) gebruiken om binnenkomende webhook-payloads tijdens de ontwikkeling te inspecteren.

## Evenementtypen

- **Create Event**: Wordt geactiveerd wanneer een nieuwe reactie wordt aangemaakt. Standaardmethode: PUT
- **Update Event**: Wordt geactiveerd wanneer een reactie wordt bewerkt. Standaardmethode: PUT
- **Delete Event**: Wordt geactiveerd wanneer een reactie wordt verwijderd. Standaardmethode: DELETE

Elk evenement bevat de volledige reactiedata in de verzoekbody (zie [Gegevensstructuren](/guide-webhooks.html#webhooks-structures) voor het payload-formaat).