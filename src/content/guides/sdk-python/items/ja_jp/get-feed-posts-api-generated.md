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

戻り値: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts200_response.py)

## 例

[inline-code-attrs-start title = 'get_feed_posts の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts200_response import GetFeedPosts200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、省略した場合のデフォルトは https://fastcomments.com です
# サポートされている構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを API サーバのセキュリティポリシーに従って構成する必要があります。
# 以下に各認証方式の例を示します。
# 下の例から
# あなたの認証ユースケースに合うものを選んで使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (オプション)
    limit = 56 # int |  (オプション)
    tags = ['tags_example'] # List[str] |  (オプション)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, after_id=after_id, limit=limit, tags=tags)
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]