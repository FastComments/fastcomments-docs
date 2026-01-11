Nell'admin dei Webhooks ci sono pulsanti `Send Test Payload` per ogni tipo di evento (Create, Update, Delete). Gli eventi Create e Update inviano un oggetto WebhookComment fittizio, mentre il test di Delete invierà un corpo della richiesta fittizio contenente solo un ID.

Il test effettuerà due chiamate per verificare il codice di risposta per gli scenari "happy" (API Key corretta) e "sad" (API Key non valida).

Quando il test invia una API Key non valida dovresti restituire un codice di stato 401 affinché il test sia considerato completamente superato. Se non controlli correttamente il valore del token, vedrai un errore.

Questo serve a garantire che tu autentichi correttamente la richiesta.