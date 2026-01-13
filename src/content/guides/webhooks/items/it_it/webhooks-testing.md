Nella Webhooks admin ci sono `Send Test Payload` buttons per ogni tipo di evento (Create, Update, Delete). Gli eventi Create e Update inviano un oggetto di prova WebhookComment, mentre il test di Delete invierà un corpo della richiesta fittizio contenente solo un ID.

## Verifica dei payload

Quando testi la tua integrazione webhook, verifica che le richieste in arrivo includano le seguenti intestazioni:

1. **`token`** - Il tuo API Secret
2. **`X-FastComments-Timestamp`** - Unix timestamp (secondi)
3. **`X-FastComments-Signature`** - HMAC-SHA256 signature

Usa la verifica della firma HMAC per garantire che i payload siano autentici.

## Strumenti di test

Puoi usare strumenti come [webhook.site](https://webhook.site) o [ngrok](https://ngrok.com) per ispezionare i payload webhook in arrivo durante lo sviluppo.

## Tipi di evento

- **Create Event**: Scatenato quando viene creato un nuovo commento. Metodo predefinito: PUT
- **Update Event**: Scatenato quando un commento viene modificato. Metodo predefinito: PUT
- **Delete Event**: Scatenato quando un commento viene eliminato. Metodo predefinito: DELETE

Ogni evento include tutti i dati del commento nel corpo della richiesta (vedi [Strutture dei dati](/guides/webhooks/webhooks-structures) per il formato del payload).