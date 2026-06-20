## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | パス | 必須 |  |
| broadcastId | string | クエリ | 任意 |  |
| sso | string | クエリ | 任意 |  |

## Response

戻り値: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_post_response.py)

## 例

[inline-code-attrs-start title = 'create_feed_post_public の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_post_response import CreateFeedPostResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスでコンテキストを作成します
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  （任意）
    sso = 'sso_example' # str |  （任意）

    try:
        api_response = api_instance.create_feed_post_public(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->create_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_feed_post_public: %s\n" % e)
[inline-code-end]

---