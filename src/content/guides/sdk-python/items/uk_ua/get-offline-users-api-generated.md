Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Ідентифікатор URL сторінки (очищений на сервері). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Тай-брейкер курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб уникнути пропуску записів через однакові імена. |

## Відповідь

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Приклад

[inline-code-attrs-start title = 'get_offline_users Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хосту є необов’язковим і за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрити контекст з екземпляром API клієнта
with client.ApiClient(configuration) as api_client:
    # Створити екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Ідентифікатор URL сторінки (очищений на сервері).
    after_name = 'after_name_example' # str | Курсор: передайте nextAfterName з попередньої відповіді. (необов’язково)
    after_user_id = 'after_user_id_example' # str | Тай-брейкер курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли afterName встановлений, щоб уникнути пропуску записів через однакові імена. (необов’язково)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]