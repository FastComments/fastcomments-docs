---
### 執行單元測試

單元測試涵蓋 SSO 功能：

```bash
swift test --filter SSOTests
```

### 執行整合測試

整合測試需要先設定環境變數：

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```
---