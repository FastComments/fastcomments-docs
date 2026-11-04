Die neuen Webhook-Seiten zum Erstellen und Bearbeiten haben eine `Send Test Payload`-Schaltfläche, die eine Anfrage an die aktuell im Formular befindliche URL sendet, unabhängig davon, ob sie gespeichert wurde oder nicht. Die Create- und Update-Events senden ein Dummy-WebhookComment-Objekt, während beim Testen von Delete ein Dummy-Request-Body mit nur einer ID gesendet wird.

## Verifizierung von Payloads

Beim Testen Ihrer Webhook-Integration sollten Sie überprüfen, ob die eingehenden Anfragen die folgenden Header enthalten:

1. **`X-FastComments-Timestamp`** - Unix-Zeitstempel (Sekunden)
2. **`X-FastComments-Signature`** - HMAC-SHA256-Signatur

Webhooks, die vor Einführung des Signaturschemas erstellt wurden, erhalten außerdem einen **`token`**-Header, der Ihr API-Secret enthält. Neue Webhooks nicht.

Verwenden Sie die HMAC-Signaturverifizierung, um sicherzustellen, dass die Payloads authentisch sind.

## Test-Tools

Sie können Tools wie [webhook.site](https://webhook.site) oder [ngrok](https://ngrok.com) verwenden, um eingehende Webhook-Payloads während der Entwicklung zu inspizieren.

## Event-Typen

- **Create-Event**: Wird ausgelöst, wenn ein neuer Kommentar erstellt wird.
- **Update-Event**: Wird ausgelöst, wenn ein Kommentar bearbeitet wird.
- **Delete-Event**: Wird ausgelöst, wenn ein Kommentar gelöscht wird.

Jeder Webhook ist an ein Event und eine HTTP-Methode (POST, PUT oder DELETE) gebunden. Jedes Event enthält die vollständigen Kommentardaten im Request-Body (siehe [Datenstrukturen](/guide-webhooks.html#webhooks-structures) für das Payload-Format).