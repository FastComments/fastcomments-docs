## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | No |  |
| commentId | string | query | No |  |
| sso | string | query | No |  |

## 回應

回傳: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_manual_badges_response.py)

## 範例

[inline-code-attrs-start title = 'get_manual_badges_for_user 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_manual_badges_response import GetUserManualBadgesResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援之設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 ApiClient 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的一個實例
    api_instance = client.ModerationApi(api_client)
    badges_user_id = 'badges_user_id_example' # str |  (選用)
    comment_id = 'comment_id_example' # str |  (選用)
    sso = 'sso_example' # str |  (選用)

    try:
        api_response = api_instance.get_manual_badges_for_user(badges_user_id=badges_user_id, comment_id=comment_id, sso=sso)
        print("The response of ModerationApi->get_manual_badges_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_manual_badges_for_user: %s\n" % e)
[inline-code-end]