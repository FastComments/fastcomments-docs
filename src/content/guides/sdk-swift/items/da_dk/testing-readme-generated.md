### Kørsel af enhedstests

Enhedstests dækker SSO-funktionaliteten:

```bash
swift test --filter SSOTests
```

### Kørsel af integrationstests

Integrationstests kræver, at miljøvariabler er sat:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```