## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | パス | はい |  |
| search | string | クエリ | はい |  |
| locale | string | クエリ | いいえ |  |
| rating | string | クエリ | いいえ |  |
| page | number | クエリ | いいえ |  |

## レスポンス

返却: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_search_response.py)

## 例

[inline-code-attrs-start title = 'get_gifs_search 例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetGifsSearchOptions
from client.models.get_gifs_search_response import GetGifsSearchResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py にはサポートされているすべての構成パラメータの一覧があります
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクラスのインスタンスを作成
with client.ApiClient(configuration) as api_client:
    # Enter a context with an instance of the API client
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    search = 'search_example' # str | 
    locale = 'locale_example' # str |  (optional)
    rating = 'rating_example' # str |  (optional)
    page = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_gifs_search(tenant_id, search, GetGifsSearchOptions(locale=locale, rating=rating, page=page))
        print("The response of PublicApi->get_gifs_search:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_search: %s\n" % e)
[inline-code-end]