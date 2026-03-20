Im Webhooks-Admin gibt es für jeden Ereignistyp `Send Test Payload`-Schaltflächen (Erstellen, Aktualisieren, Löschen). Die Erstellen- und Aktualisieren-Ereignisse senden ein Dummy-WebhookComment-Objekt, während beim Testen des Lösch-Ereignisses ein Dummy-Anfragekörper mit nur einer ID gesendet wird.

## Überprüfung der Payloads

Prüfen Sie beim Testen Ihrer Webhook-Integration, ob die eingehenden Anfragen die folgenden Header enthalten:

1. **`token`** - Ihr API-Secret
2. **`X-FastComments-Timestamp`** - Unix-Zeitstempel (Sekunden)
3. **`X-FastComments-Signature`** - HMAC-SHA256-Signatur

Verwenden Sie die HMAC-Signaturüberprüfung, um sicherzustellen, dass die Payloads authentisch sind.

## Test-Tools

Sie können Tools wie [webhook.site](https://webhook.site) oder [ngrok](https://ngrok.com) verwenden, um eingehende Webhook-Payloads während der Entwicklung einzusehen.

## Ereignistypen

- **Erstellen-Ereignis**: Wird ausgelöst, wenn ein neuer Kommentar erstellt wird. Standardmethode: PUT
- **Aktualisieren-Ereignis**: Wird ausgelöst, wenn ein Kommentar bearbeitet wird. Standardmethode: PUT
- **Löschen-Ereignis**: Wird ausgelöst, wenn ein Kommentar gelöscht wird. Standardmethode: DELETE

Jedes Ereignis enthält die vollständigen Kommentardaten im Anfragekörper (siehe [Datenstrukturen](/guide-webhooks.html#webhooks-structures) für das Payload-Format).