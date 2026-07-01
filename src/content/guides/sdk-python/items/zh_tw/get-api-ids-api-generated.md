## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| afterId | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## 回應

Returns: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## 範例

[inline-code-attrs-start title = '取得 API IDs 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetApiIdsOptions
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參考 configuration.py 以獲取所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 API 客戶端的實例建立上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    text_search = 'text_search_example' # str |  （可選）
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  （可選）
    filters = 'filters_example' # str |  （可選）
    search_filters = 'search_filters_example' # str |  （可選）
    after_id = 'after_id_example' # str |  （可選）
    demo = True # bool |  （可選）
    sso = 'sso_example' # str |  （可選）

    try:
        api_response = api_instance.get_api_ids(tenant_id, GetApiIdsOptions(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso))
        print("The response of ModerationApi->get_api_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_ids: %s\n" % e)
[inline-code-end]