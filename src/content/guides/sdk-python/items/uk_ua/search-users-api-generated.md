## Parameters

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| usernameStartsWith | string | query | Ні |  |
| mentionGroupIds | array | query | Ні |  |
| sso | string | query | Ні |  |
| searchSection | string | query | Ні |  |

## Response

Повертає: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users_result.py)

## Example

[inline-code-attrs-start title = 'search_users Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import SearchUsersOptions
from client.models.search_users_result import SearchUsersResult
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням встановлює https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрити контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створити екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (optional)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (optional)
    sso = 'sso_example' # str |  (optional)
    search_section = 'search_section_example' # str |  (optional)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, SearchUsersOptions(username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section))
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]