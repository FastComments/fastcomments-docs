Nella sezione di amministrazione Webhooks ci sono pulsanti `Send Test Payload` per ogni tipo di evento (Create, Update, Delete). Gli eventi Create e Update inviano un oggetto di prova WebhookComment, mentre il test dell'evento Delete invierà un corpo della richiesta di prova contenente solo un ID.

## Verifica dei payload

Durante il test dell'integrazione webhook, verifica che le richieste in arrivo includano le seguenti intestazioni:

1. **`token`** - Il tuo segreto API
2. **`X-FastComments-Timestamp`** - timestamp Unix (secondi)
3. **`X-FastComments-Signature`** - firma HMAC-SHA256

Utilizza la verifica della firma HMAC per assicurarti che i payload siano autentici.

## Strumenti di test

Puoi usare strumenti come [webhook.site](https://webhook.site) o [ngrok](https://ngrok.com) per ispezionare i payload webhook in arrivo durante lo sviluppo.

## Tipi di eventi

- **Create Event**: Scatenato quando viene creato un nuovo commento. Metodo predefinito: PUT
- **Update Event**: Scatenato quando un commento viene modificato. Metodo predefinito: PUT
- **Delete Event**: Scatenato quando un commento viene eliminato. Metodo predefinito: DELETE

Ogni evento include l'intero set di dati del commento nel corpo della richiesta (vedi [Strutture dei dati](/guide-webhooks.html#webhooks-structures) per il formato del payload).

---