### Running Unit Tests

Unit tests cover the SSO functionality:

```bash
swift test --filter SSOTests
```

### Running Integration Tests

Integration tests require environment variables to be set:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```