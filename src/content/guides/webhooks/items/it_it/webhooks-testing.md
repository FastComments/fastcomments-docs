Le nuove pagine di webhook e quelle di modifica hanno un pulsante `Send Test Payload` che invia una richiesta all'URL attualmente presente nel modulo, sia che sia stato salvato o meno. Gli eventi Create e Update inviano un oggetto WebhookComment fittizio, mentre il test Delete invierà un corpo di richiesta fittizio contenente solo un ID.

## Verifica dei payload

Durante il test della tua integrazione webhook, verifica che le richieste in arrivo includano le seguenti intestazioni:

1. **`X-FastComments-Timestamp`** - timestamp Unix (secondi)  
2. **`X-FastComments-Signature`** - firma HMAC-SHA256  

I webhook creati prima dell'introduzione dello schema di firma ricevono anche un'intestazione **`token`** contenente il tuo segreto API. I nuovi webhook no.

Utilizza la verifica della firma HMAC per garantire che i payload siano autentici.

## Strumenti di test

Puoi utilizzare strumenti come [webhook.site](https://webhook.site) o [ngrok](https://ngrok.com) per ispezionare i payload webhook in arrivo durante lo sviluppo.

## Tipi di evento

- **Create Event**: Attivato quando viene creato un nuovo commento.  
- **Update Event**: Attivato quando un commento viene modificato.  
- **Delete Event**: Attivato quando un commento viene eliminato.  

Ogni webhook è associato a un singolo evento e a un metodo HTTP (POST, PUT o DELETE). Ogni evento include i dati completi del commento nel corpo della richiesta (vedi [Data Structures](/guide-webhooks.html#webhooks-structures) per il formato del payload).

---