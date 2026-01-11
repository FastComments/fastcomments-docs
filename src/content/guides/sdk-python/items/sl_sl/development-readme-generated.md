### Zagon testov

```bash
# Nastavite spremenljivke okolja
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Zaženite teste
pytest tests/
```

### Ponovno generiranje odjemalca

Za ponovno generiranje API odjemalca iz najnovejše specifikacije OpenAPI:

```bash
./update.sh
```

To bo:
1. Preneslo najnovejšo OpenAPI specifikacijo s tekočega strežnika FastComments (ali uporabilo lokalni openapi.yaml)
2. Ustvarilo Python odjemalsko kodo
3. Poenostavilo strukturo imenikov
4. Odstranilo odvečne konfiguracijske datoteke