## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| page | number | query | いいえ |  |
| count | number | query | いいえ |  |
| text-search | string | query | いいえ |  |
| byIPFromComment | string | query | いいえ |  |
| filters | string | query | いいえ |  |
| searchFilters | string | query | いいえ |  |
| sorts | string | query | いいえ |  |
| demo | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comments_response.py)

## 例

[inline-code-attrs-start title = 'get_api_comments の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comments_response import ModerationAPIGetCommentsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスでコンテキストに入る
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成する
    api_instance = client.ModerationApi(api_client)
    page = 3.4 # float | （オプション）
    count = 3.4 # float | （オプション）
    text_search = 'text_search_example' # str | （オプション）
    by_ip_from_comment = 'by_ip_from_comment_example' # str | （オプション）
    filters = 'filters_example' # str | （オプション）
    search_filters = 'search_filters_example' # str | （オプション）
    sorts = 'sorts_example' # str | （オプション）
    demo = True # bool | （オプション）
    sso = 'sso_example' # str | （オプション）

    try:
        api_response = api_instance.get_api_comments(page=page, count=count, text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_comments: %s\n" % e)
[inline-code-end]