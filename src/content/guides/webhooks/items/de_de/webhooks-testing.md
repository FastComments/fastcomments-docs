Im Webhooks-Admin gibt es `Send Test Payload`-Schaltflächen für jeden Ereignistyp (Create, Update, Delete). Die Create- und Update-Ereignisse senden ein Dummy WebhookComment-Objekt, während beim Testen von Delete ein Dummy-Anfragekörper mit nur einer ID gesendet wird.

## Überprüfung der Payloads

When testing your webhook integration, verify the incoming requests include the following headers:

1. **`token`** - Ihr API-Secret
2. **`X-FastComments-Timestamp`** - Unix-Zeitstempel (Sekunden)
3. **`X-FastComments-Signature`** - HMAC-SHA256-Signatur

Verwenden Sie die HMAC-Signaturprüfung, um sicherzustellen, dass die Payloads authentisch sind.

## Test-Tools

Sie können Werkzeuge wie [webhook.site](https://webhook.site) oder [ngrok](https://ngrok.com) verwenden, um eingehende Webhook-Payloads während der Entwicklung zu untersuchen.

## Ereignistypen

- **Create Event**: Wird ausgelöst, wenn ein neuer Kommentar erstellt wird. Standardmethode: PUT
- **Update Event**: Wird ausgelöst, wenn ein Kommentar bearbeitet wird. Standardmethode: PUT
- **Delete Event**: Wird ausgelöst, wenn ein Kommentar gelöscht wird. Standardmethode: DELETE

Jedes Ereignis enthält die vollständigen Kommentardaten im Anfragekörper (siehe [Datenstrukturen](/guides/webhooks/webhooks-structures) für das Payload-Format).