## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sorts | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳： [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_response.py)

## 範例

[inline-code-attrs-start title = 'post_api_export 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_response import ModerationExportResponse
from client.rest import ApiException
from pprint import pprint

# 設定 host 為可選，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (選用)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (選用)
    filters = 'filters_example' # str |  (選用)
    search_filters = 'search_filters_example' # str |  (選用)
    sorts = 'sorts_example' # str |  (選用)
    sso = 'sso_example' # str |  (選用)

    try:
        api_response = api_instance.post_api_export(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, sso=sso)
        print("The response of ModerationApi->post_api_export:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_api_export: %s\n" % e)
[inline-code-end]