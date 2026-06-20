## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| value | string | query | いいえ |  |
| filters | string | query | いいえ |  |
| searchFilters | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_comment_search_response.py)

## 例

[inline-code-attrs-start title = 'get_search_comments_summary の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_comment_search_response import ModerationCommentSearchResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# configuration.py にはサポートされているすべての設定パラメータの一覧があります。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (任意)
    filters = 'filters_example' # str |  (任意)
    search_filters = 'search_filters_example' # str |  (任意)
    sso = 'sso_example' # str |  (任意)

    try:
        api_response = api_instance.get_search_comments_summary(value=value, filters=filters, search_filters=search_filters, sso=sso)
        print("The response of ModerationApi->get_search_comments_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_comments_summary: %s\n" % e)
[inline-code-end]