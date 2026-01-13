Агрегирует документы, группируя их (если задан groupBy) и применяя несколько операций.
Поддерживаются различные операции (например, sum, countDistinct, avg и т.д.).

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| parentTenantId | string | query | Нет |  |
| includeStats | boolean | query | Нет |  |

## Ответ

Возвращает: [`AggregationResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregation_response.py)

## Пример

[inline-code-attrs-start title = 'aggregate Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregation_request import AggregationRequest
from client.models.aggregation_response import AggregationResponse
from client.rest import ApiException
from pprint import pprint

# Определение хоста необязательно и по умолчанию использует https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API-сервера.
# Ниже приведены примеры для каждого метода аутентификации, используйте пример,
# который соответствует вашему сценарию аутентификации.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API-ключа, если это необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (необязательно)
    include_stats = True # bool |  (необязательно)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]