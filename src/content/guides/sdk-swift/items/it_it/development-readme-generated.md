### Rigenerazione del client

Per rigenerare il client API dalla più recente specifica OpenAPI:

1. Assicurati di avere il server FastComments in esecuzione localmente su `http://localhost:3001`
2. Esegui lo script di aggiornamento:

```bash
./update.sh
```

Questo farà:
- Scaricherà la più recente specifica OpenAPI
- Genererà il codice client Swift (con documentazione API in client/docs)
- Costruirà il pacchetto per verificare che tutto funzioni