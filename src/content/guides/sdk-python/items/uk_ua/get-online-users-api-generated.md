Користувачі, які зараз онлайн на сторінці: люди, чиї websocket-сесії підписані на цю сторінку в даний момент.
Повертає anonCount + totalCount (підписники в межах кімнати, включно з анонімними глядачами, яких ми не перераховуємо).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Page URL identifier (cleaned server-side). |
| afterName | string | query | Ні | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | Ні | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Відповідь

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Вказувати host необов'язково; за замовчуванням використовується https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрийте контекст із екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Ідентифікатор URL сторінки (очищається на сервері).
    after_name = 'after_name_example' # str | Курсор: передайте nextAfterName з попередньої відповіді. (необов'язково)
    after_user_id = 'after_user_id_example' # str | Тайбрейкер курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли встановлено afterName, щоб однакові імена не призводили до втрати записів. (необов'язково)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]