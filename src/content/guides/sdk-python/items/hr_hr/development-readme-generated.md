### Pokretanje testova

```bash
# Postavite varijable okoline
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Pokrenite testove
pytest tests/
```

### Regeneriranje klijenta

Za ponovno generiranje API klijenta iz najnovije OpenAPI specifikacije:

```bash
./update.sh
```

Ovo će:
1. Preuzeti najnoviju OpenAPI specifikaciju s pokrenutog FastComments poslužitelja (ili koristiti lokalni openapi.yaml)
2. Generirati Python klijentski kod
3. Izravnati strukturu direktorija
4. Ukloniti suvišne konfiguracijske datoteke