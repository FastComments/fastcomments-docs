Поставите потребне променљиве окружења:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Покрените тестове:

```bash
nimble test
```

Или покрените одређене тестове:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```