### Запуск тестов

```bash
# Set up environment variables
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Run tests
pytest tests/
```

### Регенерация клиента

Чтобы заново сгенерировать API-клиент из последней спецификации OpenAPI:

```bash
./update.sh
```

Это приведёт к следующему:
1. Загрузке последней спецификации OpenAPI с работающего сервера FastComments (или использованию локального файла openapi.yaml)
2. Генерации кода Python-клиента
3. Выравниванию структуры каталогов
4. Очистке избыточных файлов конфигурации