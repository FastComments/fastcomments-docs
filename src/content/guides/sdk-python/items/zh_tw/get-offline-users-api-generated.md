Past commenters on the page who are NOT currently online. Sorted by displayName.  
使用此端點於已耗盡 /users/online 後，渲染「Members」區段。  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
在 commenterName 上使用游標分頁：伺服器從 afterName 起，透過 $gt 前進走訪部分 {tenantId, urlId, commenterName} 索引，無需 $skip 成本。

## Parameters

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 頁面 URL 識別碼（在伺服器端已清理）。 |
| afterName | string | query | 否 | 游標：從前一次回應傳遞 nextAfterName。 |
| afterUserId | string | query | 否 | 游標平手解決：從前一次回應傳遞 nextAfterUserId。當 afterName 被設定時，為必填，以避免名稱相同的項目被遺漏。 |

## Response

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'get_offline_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 API 客戶端實例進入情境
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 頁面 URL 識別碼（在伺服器端已清理）。
    after_name = 'after_name_example' # str | 游標：從前一次回應傳遞 nextAfterName。（可選）
    after_user_id = 'after_user_id_example' # str | 游標平手解決：從前一次回應傳遞 nextAfterUserId。當 afterName 被設定時，為必填，以避免名稱相同的項目被遺漏。（可選）

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]

---