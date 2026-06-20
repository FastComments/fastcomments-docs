## Параметри

| Name | Тип | Location | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| badgeId | string | query | Не |  |
| type | number | query | Не |  |
| displayedOnComments | boolean | query | Не |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_user_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигуриране.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# в съответствие с политиката за сигурност на API сървъра.
# Примери за всеки метод за удостоверяване са показани по-долу, използвайте примера който
# удовлетворява вашия случай на използване за удостоверяване.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (по избор)
    badge_id = 'badge_id_example' # str |  (по избор)
    type = 3.4 # float |  (по избор)
    displayed_on_comments = True # bool |  (по избор)
    limit = 3.4 # float |  (по избор)
    skip = 3.4 # float |  (по избор)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]