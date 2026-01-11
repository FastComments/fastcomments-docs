## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |

## Ответ

Возвращает: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_subscriptions_api_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_subscriptions'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_subscriptions_api_response import GetSubscriptionsAPIResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# клиент должен настроить параметры аутентификации и авторизации
# in accordance with the API server security policy.
# в соответствии с политикой безопасности сервера API.
# Examples for each auth method are provided below, use the example that
# Примеры для каждого метода аутентификации приведены ниже, используйте тот, который
# satisfies your auth use case.
# соответствует вашему варианту использования аутентификации.

# Configure API key authorization: api_key
# Настройте авторизацию с помощью API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_subscriptions(tenant_id, user_id=user_id)
        print("The response of DefaultApi->get_subscriptions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_subscriptions: %s\n" % e)
[inline-code-end]