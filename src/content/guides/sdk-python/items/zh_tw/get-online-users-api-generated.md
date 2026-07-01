Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別碼（在伺服器端已清理）。 |
| afterName | string | query | No | 游標：傳入先前回應中的 nextAfterName。 |
| afterUserId | string | query | No | 游標平衡鍵：傳入先前回應中的 nextAfterUserId。當設定 afterName 時為必填，以避免名稱相同的項目被遺漏。 |

## Response

回傳：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## 範例

[inline-code-attrs-start title = 'get_online_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為可選，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客戶端實例建立一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 頁面 URL 識別碼（在伺服器端已清理）。
    after_name = 'after_name_example' # str | 游標：傳入先前回應中的 nextAfterName。（可選）
    after_user_id = 'after_user_id_example' # str | 游標平衡鍵：傳入先前回應中的 nextAfterUserId。當設定 afterName 時為必填，以避免名稱相同的項目被遺漏。（可選）

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]