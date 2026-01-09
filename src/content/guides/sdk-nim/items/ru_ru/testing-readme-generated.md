---
Установите необходимые переменные окружения:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Запустите тесты:

```bash
nimble test
```

Или запустите конкретные тесты:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```
---