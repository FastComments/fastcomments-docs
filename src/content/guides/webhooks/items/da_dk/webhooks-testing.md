I Webhooks-administrationen er der `Send Test Payload`-knapper for hver begivenhedstype (Create, Update, Delete). Create- og Update-begivenhederne sender et dummy WebhookComment-objekt, mens test af Delete sender en dummy request-body med kun et ID.

## Verificering af payloads

Når du tester din webhook-integration, skal du sikre, at de indkommende forespørgsler indeholder følgende headere:

1. **`token`** - Din API-secret
2. **`X-FastComments-Timestamp`** - Unix-tidsstempel (sekunder)
3. **`X-FastComments-Signature`** - HMAC-SHA256-signatur

Brug HMAC-signaturverifikation for at sikre, at payloads er autentiske.

## Testværktøjer

Du kan bruge værktøjer som [webhook.site](https://webhook.site) eller [ngrok](https://ngrok.com) til at inspicere indkommende webhook-payloads under udvikling.

## Begivenhedstyper

- **Create Event**: Udløses, når en ny kommentar oprettes. Standardmetode: PUT
- **Update Event**: Udløses, når en kommentar redigeres. Standardmetode: PUT
- **Delete Event**: Udløses, når en kommentar slettes. Standardmetode: DELETE

Hver begivenhed indeholder de fulde kommentardata i request-bodyen (se [Datastrukturer](/guide-webhooks.html#webhooks-structures) for payloadformatet).