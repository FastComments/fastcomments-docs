Setzen Sie die erforderlichen Umgebungsvariablen:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Führen Sie die Tests aus:

```bash
nimble test
```

Oder führen Sie bestimmte Tests aus:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```