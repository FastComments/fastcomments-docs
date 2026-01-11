### Esecuzione dei test

```bash
# Impostare le variabili d'ambiente
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Eseguire i test
pytest tests/
```

### Rigenerazione del client

Per rigenerare il client API dalla più recente specifica OpenAPI:

```bash
./update.sh
```

Questo farà:
1. Scaricare la specifica OpenAPI più recente da un server FastComments in esecuzione (o usare openapi.yaml locale)
2. Generare il codice client Python
3. Appiattire la struttura delle directory
4. Rimuovere i file di configurazione ridondanti