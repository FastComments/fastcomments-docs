### Запуск тестов

```bash
# Set up environment variables
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Run tests
pytest tests/
```

### Regenerating the Client

Чтобы регенерировать клиент API из последней спецификации OpenAPI:

```bash
./update.sh
```

Это выполнит:
1. Загрузит последнюю спецификацию OpenAPI с работающего сервера FastComments (или использовать локальный openapi.yaml)
2. Сгенерирует код клиента на Python
3. Упростит структуру каталогов
4. Удалит избыточные файлы конфигурации