---
Defina as variáveis de ambiente necessárias:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Execute os testes:

```bash
nimble test
```

Ou execute testes específicos:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```
---