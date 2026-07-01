## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| badgeId | string | query | Yes |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 応答

Returns: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/remove_user_badge_response.py)

## 例

[inline-code-attrs-start title = 'put_remove_badge の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PutRemoveBadgeOptions
from client.models.remove_user_badge_response import RemoveUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py を参照してください。サポートされているすべての設定パラメータの一覧があります。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (オプション)
    comment_id = 'comment_id_example' # str |  (オプション)
    broadcast_id = 'broadcast_id_example' # str |  (オプション)
    sso = 'sso_example' # str |  (オプション)

    try:
        api_response = api_instance.put_remove_badge(tenant_id, badge_id, PutRemoveBadgeOptions(user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->put_remove_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_remove_badge: %s\n" % e)
[inline-code-end]