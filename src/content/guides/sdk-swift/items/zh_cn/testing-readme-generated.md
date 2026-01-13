### 运行单元测试

单元测试涵盖 SSO 功能：

```bash
swift test --filter SSOTests
```

### 运行集成测试

集成测试需要设置环境变量：

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```