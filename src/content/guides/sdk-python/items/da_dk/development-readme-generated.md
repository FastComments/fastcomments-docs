### Kør tests

```bash
# Opsæt miljøvariabler
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Kør tests
pytest tests/
```

### Regenerer klienten

For at regenerere API-klienten fra den seneste OpenAPI-specifikation:

```bash
./update.sh
```

Dette vil:
1. Download den seneste OpenAPI-specifikation fra en kørende FastComments-server (eller brug lokal openapi.yaml)
2. Generer Python-klientkoden
3. Fladgør mappestrukturen
4. Ryd op i overflødige konfigurationsfiler