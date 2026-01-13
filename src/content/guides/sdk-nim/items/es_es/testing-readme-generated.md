---
Establezca las variables de entorno requeridas:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Ejecute las pruebas:

```bash
nimble test
```

O ejecute pruebas específicas:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```
---