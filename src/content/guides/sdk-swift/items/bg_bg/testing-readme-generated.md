### Стартиране на юнит тестове

Юнит тестовете покриват функционалността на SSO:

```bash
swift test --filter SSOTests
```

### Стартиране на интеграционни тестове

Интеграционните тестове изискват задаване на променливи на средата:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```