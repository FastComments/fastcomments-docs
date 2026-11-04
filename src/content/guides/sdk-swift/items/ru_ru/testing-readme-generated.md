### Запуск модульных тестов

Модульные тесты покрывают функциональность SSO:

```bash
swift test --filter SSOTests
```

### Запуск интеграционных тестов

Для интеграционных тестов необходимо установить переменные окружения:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```