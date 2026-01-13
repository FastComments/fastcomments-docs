req
tenantId
afterId

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| afterId | string | query | いいえ |  |
| limit | integer | query | いいえ |  |
| tags | array | query | いいえ |  |
| sso | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |
| includeUserInfo | boolean | query | いいえ |  |

## レスポンス

戻り値: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_public200_response.py)

## 例

[inline-code-attrs-start title = 'get_feed_posts_public の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts_public200_response import GetFeedPostsPublic200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、既定は https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使用してコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (任意)
    limit = 56 # int |  (任意)
    tags = ['tags_example'] # List[str] |  (任意)
    sso = 'sso_example' # str |  (任意)
    is_crawler = True # bool |  (任意)
    include_user_info = True # bool |  (任意)

    try:
        api_response = api_instance.get_feed_posts_public(tenant_id, after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info)
        print("The response of PublicApi->get_feed_posts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_public: %s\n" % e)
[inline-code-end]