## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| afterId | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## 範例

[inline-code-attrs-start title = 'get_api_ids 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# 設定 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援之設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 ApiClient 實例開啟一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (可選)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (可選)
    filters = 'filters_example' # str |  (可選)
    search_filters = 'search_filters_example' # str |  (可選)
    after_id = 'after_id_example' # str |  (可選)
    demo = True # bool |  (可選)
    sso = 'sso_example' # str |  (可選)

    try:
        api_response = api_instance.get_api_ids(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_ids: %s\n" % e)
[inline-code-end]