## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | パス | はい |  |
| postId | string | パス | はい |  |
| broadcastId | string | クエリ | いいえ |  |
| sso | string | クエリ | いいえ |  |

## Response

返却: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_post_response.py)

## 例

[inline-code-attrs-start title = 'update_feed_post_public 例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UpdateFeedPostPublicOptions
from client.models.create_feed_post_response import CreateFeedPostResponse
from client.models.update_feed_post_params import UpdateFeedPostParams
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# configuration.py でサポートされているすべての設定パラメータの一覧を確認してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    update_feed_post_params = client.UpdateFeedPostParams() # UpdateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  （オプション）
    sso = 'sso_example' # str |  （オプション）

    try:
        api_response = api_instance.update_feed_post_public(tenant_id, post_id, update_feed_post_params, UpdateFeedPostPublicOptions(broadcast_id=broadcast_id, sso=sso))
        print("The response of PublicApi->update_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_feed_post_public: %s\n" % e)
[inline-code-end]

---