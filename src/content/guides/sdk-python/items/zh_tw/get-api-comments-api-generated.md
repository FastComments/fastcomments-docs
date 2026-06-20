## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | 否 |  |
| count | number | query | 否 |  |
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sorts | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comments_response.py)

## 範例

[inline-code-attrs-start title = 'get_api_comments 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comments_response import ModerationAPIGetCommentsResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    page = 3.4 # float |  (可選)
    count = 3.4 # float |  (可選)
    text_search = 'text_search_example' # str |  (可選)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (可選)
    filters = 'filters_example' # str |  (可選)
    search_filters = 'search_filters_example' # str |  (可選)
    sorts = 'sorts_example' # str |  (可選)
    demo = True # bool |  (可選)
    sso = 'sso_example' # str |  (可選)

    try:
        api_response = api_instance.get_api_comments(page=page, count=count, text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_comments: %s\n" % e)
[inline-code-end]