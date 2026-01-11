## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| badgeId | string | query | Не |  |
| type | number | query | Не |  |
| displayedOnComments | boolean | query | Не |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badges200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_user_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badges200_response import GetUserBadges200Response
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и разрешение
# в съответствие с политиката за сигурност на API сървъра.
# Примерите за всеки метод за удостоверяване са предоставени по-долу; използвайте примера, който
# отговаря на вашия случай на употреба за удостоверяване.

# Конфигуриране на авторизация с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Премахнете коментара по-долу, за да зададете префикс (напр. Bearer) за API ключ, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създаване на инстанция на API класа
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