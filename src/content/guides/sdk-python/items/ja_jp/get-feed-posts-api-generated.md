req
tenantId
afterId

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| afterId | string | query | いいえ |  |
| limit | integer | query | いいえ |  |
| tags | array | query | いいえ |  |

## レスポンス

返却値: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## 例

[inline-code-attrs-start title = 'get_feed_posts の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts_response import GetFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可パラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 各認証方法の例は以下に示されています。
# 自分の認証ユースケースに合った例を使用してください。

# API キー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて以下のコメントを外し、API キーのプレフィックス（例: Bearer）を設定してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスのコンテキストに入る
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (任意)
    limit = 56 # int |  (任意)
    tags = ['tags_example'] # List[str] |  (任意)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, after_id=after_id, limit=limit, tags=tags)
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]