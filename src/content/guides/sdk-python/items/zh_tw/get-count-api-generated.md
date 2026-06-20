---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filter | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_count_comments_response.py)

## 範例

[inline-code-attrs-start title = 'get_count 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_count_comments_response import ModerationAPICountCommentsResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (可選)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (可選)
    filter = 'filter_example' # str |  (可選)
    search_filters = 'search_filters_example' # str |  (可選)
    demo = True # bool |  (可選)
    sso = 'sso_example' # str |  (可選)

    try:
        api_response = api_instance.get_count(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filter=filter, search_filters=search_filters, demo=demo, sso=sso)
        print("The response of ModerationApi->get_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_count: %s\n" % e)
[inline-code-end]

---