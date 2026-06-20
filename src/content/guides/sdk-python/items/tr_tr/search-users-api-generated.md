## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| usernameStartsWith | string | query | Hayır |  |
| mentionGroupIds | array | query | Hayır |  |
| sso | string | query | Hayır |  |
| searchSection | string | query | Hayır |  |

## Yanıt

Döndürür: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users_result.py)

## Örnek

[inline-code-attrs-start title = 'search_users Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.search_users_result import SearchUsersResult
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
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (isteğe bağlı)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)
    search_section = 'search_section_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section)
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]