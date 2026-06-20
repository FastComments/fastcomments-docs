## パラメータ

| Name | Type | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| locale | string | query | いいえ |  |
| rating | string | query | いいえ |  |
| page | number | query | いいえ |  |

## レスポンス

戻り値: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_trending_response.py)

## 例

[inline-code-attrs-start title = 'get_gifs_trending の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_trending_response import GetGifsTrendingResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    locale = 'locale_example' # str |  （任意）
    rating = 'rating_example' # str |  （任意）
    page = 3.4 # float |  （任意）

    try:
        api_response = api_instance.get_gifs_trending(tenant_id, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_trending:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_trending: %s\n" % e)
[inline-code-end]