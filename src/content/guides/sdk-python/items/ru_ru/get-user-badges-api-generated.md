## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Ответ

Возвращает: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## Пример

[inline-code-attrs-start title = 'get_user_badges Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# Клиент должен настроить параметры аутентификации и авторизации
# in accordance with the API server security policy.
# в соответствии с политикой безопасности сервера API.
# Examples for each auth method are provided below, use the example that
# Примеры для каждого метода аутентификации приведены ниже, используйте пример, который
# satisfies your auth use case.
# соответствует вашему варианту использования аутентификации.

# Configure API key authorization: api_key
# Настроить авторизацию по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API-ключа, если нужно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (необязательно)
    badge_id = 'badge_id_example' # str |  (необязательно)
    type = 3.4 # float |  (необязательно)
    displayed_on_comments = True # bool |  (необязательно)
    limit = 3.4 # float |  (необязательно)
    skip = 3.4 # float |  (необязательно)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]