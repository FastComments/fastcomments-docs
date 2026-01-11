### Запуск тестів

```bash
# Налаштуйте змінні середовища
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Запустіть тести
pytest tests/
```

### Перегенерація клієнта

Щоб перегенерувати клієнта API з останньої специфікації OpenAPI:

```bash
./update.sh
```

Це виконає:
1. Завантажить останню специфікацію OpenAPI з працюючого сервера FastComments (або використає локальний openapi.yaml)
2. Згенерує код клієнта для Python
3. Вирівняє структуру каталогів
4. Прибере зайві конфігураційні файли