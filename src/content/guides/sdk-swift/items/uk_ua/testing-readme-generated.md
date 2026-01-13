### Запуск модульних тестів

Модульні тести охоплюють функціональність SSO:

```bash
swift test --filter SSOTests
```

### Запуск інтеграційних тестів

Інтеграційні тести вимагають встановлення змінних середовища:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```