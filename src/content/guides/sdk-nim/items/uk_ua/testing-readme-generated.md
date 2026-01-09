Встановіть необхідні змінні середовища:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Запустіть тести:

```bash
nimble test
```

Або запустіть конкретні тести:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```