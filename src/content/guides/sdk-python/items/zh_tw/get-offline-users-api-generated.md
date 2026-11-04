頁面上目前不在線上的過往留言者。按 displayName 排序。
在用完 /users/online 之後，使用此端點來渲染「成員」區塊。
基於 commenterName 的游標分頁：伺服器透過 $gt 從 afterName 開始沿著部分索引 {tenantId, urlId, commenterName} 向後遍歷，沒有 $skip 的開銷。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 頁面 URL 識別碼（由伺服器端清理）。 |
| afterName | string | query | 否 | 游標：傳入上一個回應中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游標決勝欄位：傳入上一個回應中的 nextAfterUserId。當設定 afterName 時為必填，以避免同名情況下遺漏條目。 |

## 回應

回傳：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## 範例

[inline-code-attrs-start title = 'get_offline_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
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
    url_id = 'url_id_example' # str | 頁面 URL 識別碼（由伺服器端清理）。
    after_name = 'after_name_example' # str | 游標：傳入上一個回應中的 nextAfterName。 (optional)
    after_user_id = 'after_user_id_example' # str | 游標決勝欄位：傳入上一個回應中的 nextAfterUserId。當設定 afterName 時為必填，以避免同名情況下遺漏條目。 (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]