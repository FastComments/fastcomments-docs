Gerekli ortam değişkenlerini ayarlayın:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Testleri çalıştırın:

```bash
nimble test
```

Veya belirli testleri çalıştırın:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```