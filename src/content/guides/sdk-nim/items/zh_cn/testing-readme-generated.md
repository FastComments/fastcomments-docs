设置所需的环境变量：

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

运行测试：

```bash
nimble test
```

或运行特定测试：

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```