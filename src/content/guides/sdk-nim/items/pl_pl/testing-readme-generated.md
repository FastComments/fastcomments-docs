Ustaw wymagane zmienne środowiskowe:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Uruchom testy:

```bash
nimble test
```

Lub uruchom konkretne testy:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```