## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_banned_users_from_comment_response.py)

## 範例

[inline-code-attrs-start title = 'get_ban_users_from_comment 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_banned_users_from_comment_response import GetBannedUsersFromCommentResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參考 configuration.py 取得所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 API 客戶端實例建立一個 context
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (可選)

    try:
        api_response = api_instance.get_ban_users_from_comment(tenant_id, comment_id, sso=sso)
        print("ModerationApi->get_ban_users_from_comment 的回應:\n")
        pprint(api_response)
    except Exception as e:
        print("呼叫 ModerationApi->get_ban_users_from_comment 時發生例外: %s\n" % e)
[inline-code-end]