I Webhooks-admin er der `Send Test Payload`-knapper for hver hændelsestype (Create, Update, Delete). Create- og Update-hændelserne sender et dummy WebhookComment-objekt, mens test af Delete sender en dummy request body med kun et ID.

## Bekræftelse af payloads

Når du tester din webhook-integration, skal du kontrollere, at de indkommende forespørgsler indeholder følgende headers:

1. **`token`** - Din API Secret
2. **`X-FastComments-Timestamp`** - Unix-tidsstempel (sekunder)
3. **`X-FastComments-Signature`** - HMAC-SHA256-signatur

Brug HMAC-signaturverificering for at sikre, at payloads er autentiske.

## Testværktøjer

Du kan bruge værktøjer som [webhook.site](https://webhook.site) eller [ngrok](https://ngrok.com) til at inspicere indkommende webhook-payloads under udvikling.

## Hændelsestyper

- **Create Event**: Udløses, når en ny kommentar oprettes. Default method: PUT
- **Update Event**: Udløses, når en kommentar redigeres. Default method: PUT
- **Delete Event**: Udløses, når en kommentar slettes. Default method: DELETE

Hver hændelse inkluderer hele kommentardataene i request body'en (se [Datastrukturer](/guides/webhooks/webhooks-structures) for payload-formatet).