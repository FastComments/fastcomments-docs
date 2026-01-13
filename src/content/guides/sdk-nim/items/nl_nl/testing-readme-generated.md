Stel de vereiste omgevingsvariabelen in:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Voer de tests uit:

```bash
nimble test
```

Of voer specifieke tests uit:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```