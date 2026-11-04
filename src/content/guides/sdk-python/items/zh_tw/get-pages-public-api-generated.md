列出租戶的頁面。由 FChat 桌面用戶端用來填充其房間列表。
每個頁面的已解析自訂設定中必須將 `enableFChat` 設為 true。
需要 SSO 的頁面會根據請求使用者的群組存取權限進行篩選。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 由先前請求以 `nextCursor` 回傳的不透明分頁游標。與相同的 `sortBy` 綁定。 |
| limit | integer | query | 否 | 1..200,預設 50 |
| q | string | query | 否 | 選填的、不區分大小寫的標題前綴篩選條件。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`(預設,最新的在前)、`commentCount`(留言數最多的在前)或 `title`(按字母順序)。 |
| hasComments | boolean | query | 否 | 若為 true,僅回傳至少有一則留言的頁面。 |

## 回應

回傳:[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## 範例

[inline-code-attrs-start title = 'get_pages_public 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 由先前請求以 `nextCursor` 回傳的不透明分頁游標。與相同的 `sortBy` 綁定。 (optional)
    limit = 56 # int | 1..200,預設 50 (optional)
    q = 'q_example' # str | 選填的、不區分大小寫的標題前綴篩選條件。 (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | 排序方式。`updatedAt`(預設,最新的在前)、`commentCount`(留言數最多的在前)或 `title`(按字母順序)。 (optional)
    has_comments = True # bool | 若為 true,僅回傳至少有一則留言的頁面。 (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]