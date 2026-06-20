---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| text-search | string | query | いいえ |  |
| byIPFromComment | string | query | いいえ |  |
| filters | string | query | いいえ |  |
| searchFilters | string | query | いいえ |  |
| sorts | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_response.py)

## 例

[inline-code-attrs-start title = 'post_api_export の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_response import ModerationExportResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスを使ってコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (オプション)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (オプション)
    filters = 'filters_example' # str |  (オプション)
    search_filters = 'search_filters_example' # str |  (オプション)
    sorts = 'sorts_example' # str |  (オプション)
    sso = 'sso_example' # str |  (オプション)

    try:
        api_response = api_instance.post_api_export(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, sso=sso)
        print("The response of ModerationApi->post_api_export:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_api_export: %s\n" % e)
[inline-code-end]

---