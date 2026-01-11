### Tests uitvoeren

```bash
# Omgevingsvariabelen instellen
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Tests uitvoeren
pytest tests/
```

### De client opnieuw genereren

Om de API-client te regenereren vanaf de nieuwste OpenAPI-specificatie:

```bash
./update.sh
```

Dit zal:
1. Download de nieuwste OpenAPI-specificatie van een draaiende FastComments-server (of gebruik de lokale openapi.yaml)
2. Genereer de Python-clientcode
3. Vlak de directorystructuur af
4. Ruim overtollige configuratiebestanden op