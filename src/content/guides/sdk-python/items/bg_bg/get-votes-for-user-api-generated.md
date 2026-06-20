## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |

## Отговор

Връща: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_for_user_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_votes_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_for_user_response import GetVotesForUserResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на host е незадължително и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с правилата за сигурност на API сървъра.
# Примери за всеки метод за удостоверяване са предоставени по-долу, използвайте примера, който
# отговаря на вашия случай на използване.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Премахнете коментара отдолу, за да зададете префикс (e.g. Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id = 'user_id_example' # str |  (по избор)
    anon_user_id = 'anon_user_id_example' # str |  (по избор)

    try:
        api_response = api_instance.get_votes_for_user(tenant_id, url_id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->get_votes_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes_for_user: %s\n" % e)
[inline-code-end]