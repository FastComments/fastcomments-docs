## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| usernameStartsWith | string | query | No |  |
| mentionGroupIds | array | query | No |  |
| sso | string | query | No |  |
| searchSection | string | query | No |  |

## 回應

返回：[`SearchUsersResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users_result.py)

## 範例

[inline-code-attrs-start title = 'search_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import SearchUsersOptions
from client.models.search_users_result import SearchUsersResult
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 參考 configuration.py 以取得所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
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