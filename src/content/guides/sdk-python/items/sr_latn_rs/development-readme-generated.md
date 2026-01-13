### Pokretanje testova

```bash
# Podesi promenljive okruženja
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
1. Preuzme najnoviju OpenAPI specifikaciju sa pokrenutog FastComments servera (ili koristi lokalni openapi.yaml)
2. Generiše Python klijentski kod
3. Izravna strukturu direktorijuma
4. Ukloni redundantne konfiguracione datoteke