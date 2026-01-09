---
設定必要的環境變數：

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

執行測試：

```bash
nimble test
```

或執行特定測試：

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```
---