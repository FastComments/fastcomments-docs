## Параметри

| Ім'я | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| limit | number | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_list_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_user_badge_progress_list'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_list_response import APIGetUserBadgeProgressListResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# у відповідності з політикою безпеки API-сервера.
# Приклади для кожного методу автентифікації наведено нижче — використайте приклад, який
# відповідає вашому випадку використання.

# Configure API key authorization: api_key
# Налаштуйте авторизацію за API-ключем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Розкоментуйте рядок нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Використайте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (необов'язково)
    limit = 3.4 # float |  (необов'язково)
    skip = 3.4 # float |  (необов'язково)

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, user_id=user_id, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]