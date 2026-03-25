## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| usernameStartsWith | string | query | 否 |  |
| mentionGroupIds | array | query | 否 |  |
| sso | string | query | 否 |  |
| searchSection | string | query | 否 |  |

## 回應

回傳: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users200_response.py)

## 範例

[inline-code-attrs-start title = 'search_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.search_users200_response import SearchUsers200Response
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參見 configuration.py 以獲取所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 ApiClient 實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (可選)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (可選)
    sso = 'sso_example' # str |  (可選)
    search_section = 'search_section_example' # str |  (可選)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section)
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]