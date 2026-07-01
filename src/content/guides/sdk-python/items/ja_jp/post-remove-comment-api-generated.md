## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## レスポンス

返り値: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/post_remove_comment_api_response.py)

## 例

[inline-code-attrs-start title = 'post_remove_comment の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostRemoveCommentOptions
from client.models.post_remove_comment_api_response import PostRemoveCommentApiResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py でサポートされているすべての設定パラメータの一覧を確認できます。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str |  （オプション）
    sso = 'sso_example' # str |  （オプション）

    try:
        api_response = api_instance.post_remove_comment(tenant_id, comment_id, PostRemoveCommentOptions(broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->post_remove_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_remove_comment: %s\n" % e)
[inline-code-end]