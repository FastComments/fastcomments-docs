列出租戶的頁面。由 FChat 桌面用戶端用來填充其房間列表。
要求 `enableFChat` 在每個頁面的解析後自訂設定中必須為 true。
需要 SSO 的頁面會根據請求使用者的群組存取權進行過濾。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 從先前請求回傳的不可見分頁游標 `nextCursor`。與相同的 `sortBy` 綁定。 |
| limit | integer | query | 否 | 1..200，預設 50 |
| q | string | query | 否 | 可選的不區分大小寫的標題前綴篩選。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`（預設，最新在前）、`commentCount`（評論數最多在前），或 `title`（字母順序）。 |
| hasComments | boolean | query | 否 | 如果為 true，僅回傳至少有一則評論的頁面。 |

## 回應

回傳: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## 範例

[inline-code-attrs-start title = 'get_pages_public 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | 從先前請求回傳的不可見分頁游標，為 `nextCursor`。與相同的 `sortBy` 綁定。
  limit: 56, # Integer | 1..200，預設 50
  q: 'q_example', # String | 選用的不區分大小寫的標題前綴篩選。
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | 排序方式。`updatedAt`（預設，最新在前），`commentCount`（評論數最多在前），或 `title`（字母順序）。
  has_comments: true # Boolean | 如果為 true，僅回傳至少有一則評論的頁面。
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]