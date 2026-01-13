### ユニットテストの実行

ユニットテストは SSO 機能をカバーします：

```bash
swift test --filter SSOTests
```

### 統合テストの実行

統合テストでは環境変数の設定が必要です：

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```