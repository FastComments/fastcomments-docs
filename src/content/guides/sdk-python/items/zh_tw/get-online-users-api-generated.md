目前線上頁面觀眾：其 websocket 會話當前已訂閱該頁面的人。
回傳 anonCount + totalCount（整個房間的訂閱者，包括我們未列舉的匿名觀眾）。

## 參數

| 名稱 | 類型 | 位置 | 是否必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | Page URL identifier (cleaned server-side). |
| afterName | string | query | 否 | 游標：傳入上一次回應中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游標決勝項：傳入上一次回應中的 nextAfterUserId。當設定 afterName 時需要此欄位，以避免名稱相同導致項目被遺漏。 |

## 回應

回傳：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## 範例

[inline-code-attrs-start title = 'get_online_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 頁面 URL 識別碼（伺服器端已清理）。
    after_name = 'after_name_example' # str | 游標：傳入上一次回應中的 nextAfterName。（選填）
    after_user_id = 'after_user_id_example' # str | 游標決勝項：傳入上一次回應中的 nextAfterUserId。當設定 afterName 時需要此欄位，以避免名稱相同導致項目被遺漏。（選填）

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]