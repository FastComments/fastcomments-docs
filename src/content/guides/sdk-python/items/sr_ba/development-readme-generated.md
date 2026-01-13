---
### Pokretanje testova

```bash
# Postavi varijable okruženja
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Pokreni testove
pytest tests/
```

### Regenerisanje klijenta

Da biste regenerisali API klijenta iz najnovije OpenAPI specifikacije:

```bash
./update.sh
```

Ovo će:
1. Preuzeti najnoviju OpenAPI specifikaciju sa pokrenutog FastComments servera (ili koristiti lokalni openapi.yaml)
2. Generisati Python klijentski kod
3. Izravnati strukturu direktorijuma
4. Ukloniti suvišne konfiguracione datoteke
---