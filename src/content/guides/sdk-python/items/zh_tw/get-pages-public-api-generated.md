List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 不透明的分頁游標，從先前請求的 `nextCursor` 返回。與相同的 `sortBy` 相關聯。 |
| limit | integer | query | No | 1..200，預設 50 |
| q | string | query | No | 可選的大小寫不敏感標題前綴過濾。 |
| sortBy | string | query | No | 排序方式。`updatedAt`（預設，最新的在前），`commentCount`（評論最多的在前），或 `title`（字母順序）。 |
| hasComments | boolean | query | No | 若為 true，僅返回至少有一則評論的頁面。 |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'get_pages_public 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 不透明的分頁游標，從先前請求的 `nextCursor` 返回。與相同的 `sortBy` 相關聯。 (optional)
    limit = 56 # int | 1..200，預設 50 (optional)
    q = 'q_example' # str | 可選的大小寫不敏感標題前綴過濾。 (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | 排序方式。`updatedAt`（預設，最新的在前），`commentCount`（評論最多的在前），或 `title`（字母順序）。 (optional)
    has_comments = True # bool | 若為 true，僅返回至少有一則評論的頁面。 (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]