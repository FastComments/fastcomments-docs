## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| usernameStartsWith | string | query | Ні |  |
| mentionGroupIds | array | query | Ні |  |
| sso | string | query | Ні |  |
| searchSection | string | query | Ні |  |

## Відповідь

Повертає: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users200_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад search_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.search_users200_response import SearchUsers200Response
from client.rest import ApiException
from pprint import pprint

# Визначення host необов'язкове і за замовчуванням встановлено на https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (необов'язково)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)
    search_section = 'search_section_example' # str |  (необов'язково)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section)
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]