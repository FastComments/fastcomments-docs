Коментатори на страницата от минали посещения, които в момента НЕ са онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да визуализирате секция "Членове".
Пагинация с курсор по commenterName: сървърът обходява частичния индекс {tenantId, urlId, commenterName}
от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (почиства се от сървъра). |
| afterName | string | query | Не | Курсор: подайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Допълнителен критерий за курсора: подайте nextAfterUserId от предишния отговор. Задължителен, когато afterName е зададен, така че съвпадащите имена да не пропуснат записи. |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор на URL на страницата (почиства се от сървъра).
    after_name = 'after_name_example' # str | Курсор: подайте nextAfterName от предишния отговор. (optional)
    after_user_id = 'after_user_id_example' # str | Допълнителен критерий за курсора: подайте nextAfterUserId от предишния отговор. Задължителен, когато afterName е зададен, така че съвпадащите имена да не пропуснат записи. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]