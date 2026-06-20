## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| id | string | query | 是 |  |

## 回應

回傳: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_react_users_response.py)

## 範例

[inline-code-attrs-start title = 'get_v2_page_react_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_react_users_response import GetV2PageReactUsersResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有受支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_v2_page_react_users(tenant_id, url_id, id)
        print("The response of PublicApi->get_v2_page_react_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v2_page_react_users: %s\n" % e)
[inline-code-end]