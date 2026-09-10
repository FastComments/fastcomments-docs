Die neuen und bearbeitenden Webhook‑Seiten haben einen `Send Test Payload`‑Button, der eine Anfrage an die aktuell im Formular angegebene URL sendet, unabhängig davon, ob sie gespeichert wurde. Die **Create**‑ und **Update**‑Events senden ein Dummy‑WebhookComment‑Objekt, während beim Testen von **Delete** ein Dummy‑Request‑Body mit nur einer ID gesendet wird.

## Verifying Payloads

Beim Testen Ihrer Webhook‑Integration sollten Sie überprüfen, dass die eingehenden Anfragen die folgenden Header enthalten:

1. **`X-FastComments-Timestamp`** – Unix‑Zeitstempel (Sekunden)  
2. **`X-FastComments-Signature`** – HMAC‑SHA256‑Signatur  

Webhooks, die vor der Einführung des Signaturschemas erstellt wurden, erhalten außerdem einen **`token`**‑Header, der Ihr API‑Secret enthält. Neue Webhooks erhalten diesen nicht.

Verwenden Sie die HMAC‑Signatur‑Verifizierung, um sicherzustellen, dass Payloads authentisch sind.

## Testing Tools

Sie können Werkzeuge wie [webhook.site](https://webhook.site) oder [ngrok](https://ngrok.com) verwenden, um eingehende Webhook‑Payloads während der Entwicklung zu inspizieren.

## Event Types

- **Create Event**: Ausgelöst, wenn ein neuer Kommentar erstellt wird.  
- **Update Event**: Ausgelöst, wenn ein Kommentar bearbeitet wird.  
- **Delete Event**: Ausgelöst, wenn ein Kommentar gelöscht wird.  

Jeder Webhook ist an ein Ereignis und eine HTTP‑Methode (POST, PUT oder DELETE) gebunden. Jedes Ereignis enthält die vollständigen Kommentardaten im Request‑Body (siehe [Data Structures](/guide-webhooks.html#webhooks-structures) für das Payload‑Format).