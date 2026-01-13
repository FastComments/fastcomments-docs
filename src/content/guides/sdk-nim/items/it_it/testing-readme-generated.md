Imposta le variabili d'ambiente richieste:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Esegui i test:

```bash
nimble test
```

Oppure esegui test specifici:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```