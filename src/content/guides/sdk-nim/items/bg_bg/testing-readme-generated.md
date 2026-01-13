---
Задайте необходимите променливи на средата:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Стартирайте тестовете:

```bash
nimble test
```

Или стартирайте конкретни тестове:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```
---