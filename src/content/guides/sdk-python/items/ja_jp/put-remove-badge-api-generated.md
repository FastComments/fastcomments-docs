## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | はい |  |
| userId | string | query | いいえ |  |
| commentId | string | query | いいえ |  |
| broadcastId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/remove_user_badge_response.py)

## 例

[inline-code-attrs-start title = 'put_remove_badge の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.remove_user_badge_response import RemoveUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされている設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスでコンテキストを開きます
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成する
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (オプション)
    comment_id = 'comment_id_example' # str |  (オプション)
    broadcast_id = 'broadcast_id_example' # str |  (オプション)
    sso = 'sso_example' # str |  (オプション)

    try:
        api_response = api_instance.put_remove_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_remove_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_remove_badge: %s\n" % e)
[inline-code-end]