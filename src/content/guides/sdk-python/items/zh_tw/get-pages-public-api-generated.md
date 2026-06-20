列出租戶的頁面。由 FChat 桌面用戶端用來填充其房間列表。
需要在每個頁面的解析出的自訂設定中，`enableFChat` 為 true。
需要 SSO 的頁面會根據請求使用者的群組存取權限進行過濾。

## 參數

| 名稱 | 類型 | 位置 | 必要 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 作為不透明分頁游標，從先前的請求返回為 `nextCursor`。與相同的 `sortBy` 綁定。 |
| limit | integer | query | No | 1..200, 預設 50 |
| q | string | query | No | 可選的不區分大小寫的標題前綴過濾器。 |
| sortBy | string | query | No | 排序方式。`updatedAt` (預設，最新在前), `commentCount` (評論數最多在前), 或 `title` (字母順序)。 |
| hasComments | boolean | query | No | 若為 true，僅返回至少有一則評論的頁面。 |

## 回應

回傳: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## 範例

[inline-code-attrs-start title = 'get_pages_public 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 ApiClient 實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 不透明的分頁游標，從先前的請求返回為 `nextCursor`。與相同的 `sortBy` 綁定。 (可選)
    limit = 56 # int | 1..200, 預設 50 (可選)
    q = 'q_example' # str | 可選的不區分大小寫標題前綴過濾器。 (可選)
    sort_by = client.PagesSortBy() # PagesSortBy | 排序方式。`updatedAt` (預設，最新在前), `commentCount` (評論數最多在前), 或 `title` (字母順序)。 (可選)
    has_comments = True # bool | 若為 true，僅返回至少有一則評論的頁面。 (可選)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]