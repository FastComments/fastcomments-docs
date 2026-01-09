---
Indstil de nødvendige miljøvariabler:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Kør testene:

```bash
nimble test
```

Eller kør specifikke tests:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```
---