## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| sure | string | query | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример delete_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Определение host необязательно и по умолчанию равно https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API-сервера.
# Ниже приведены примеры для каждого метода аутентификации, используйте тот,
# который соответствует вашему сценарию использования.

# Настройка авторизации по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API-ключа, если требуется
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Входим в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создаём экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    sure = 'sure_example' # str |  (optional)

    try:
        api_response = api_instance.delete_tenant(tenant_id, id, sure=sure)
        print("The response of DefaultApi->delete_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_tenant: %s\n" % e)
[inline-code-end]