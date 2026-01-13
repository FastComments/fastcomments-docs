### Unit-tests uitvoeren

Unit-tests behandelen de SSO-functionaliteit:

```bash
swift test --filter SSOTests
```

### Integratietests uitvoeren

Integratietests vereisen dat omgevingsvariabelen zijn ingesteld:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```