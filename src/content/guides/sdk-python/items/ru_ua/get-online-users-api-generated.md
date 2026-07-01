Currently-online viewers of a page: люди, чиї websocket‑сесії підписані на сторінку прямо зараз.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Ідентифікатор URL‑сторінки (очищений на боці сервера). |
| afterName | string | query | No | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | No | Додатковий критерій курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб записи зі збігом імен не пропускались. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хосту є необов’язковим і за замовчуванням має значення https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Вхід у контекст з екземпляром API‑клієнта
with client.ApiClient(configuration) as api_client:
    # Створення екземпляра класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Ідентифікатор URL‑сторінки (очищений на боці сервера).
    after_name = 'after_name_example' # str | Курсор: передайте nextAfterName з попередньої відповіді. (optional)
    after_user_id = 'after_user_id_example' # str | Додатковий критерій курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб записи зі збігом імен не пропускались. (optional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]