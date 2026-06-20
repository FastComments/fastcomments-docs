列出租戶的頁面。由 FChat 桌面用戶端用來填充其房間列表。
需要在每個頁面的解析後自訂設定中，`enableFChat` 為 true。
需要 SSO 的頁面會根據請求者的使用者群組存取權限進行過濾。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 不透明的分頁游標，來自先前請求的 `nextCursor`。與相同的 `sortBy` 綁定。 |
| limit | integer | query | No | 1..200，預設 50 |
| q | string | query | No | 選用的大小寫不敏感標題前綴篩選條件。 |
| sortBy | string | query | No | 排序方式。`updatedAt`（預設，最新在前）、`commentCount`（留言數最多在前），或 `title`（字母順序）。 |
| hasComments | boolean | query | No | 如果為 true，僅回傳至少有一則留言的頁面。 |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍屬測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 不透明的分頁游標，來自先前請求的 `nextCursor`。與相同的 `sortBy` 綁定。 (可選)
let limit = 987 // Int | 1..200，預設 50（可選）
let q = "q_example" // String | 選用的大小寫不敏感標題前綴篩選。(可選)
let sortBy = PagesSortBy() // PagesSortBy | 排序方式。`updatedAt`（預設，最新在前）、`commentCount`（留言數最多在前）或 `title`（字母順序）。 (可選)
let hasComments = true // Bool | 如果為 true，僅回傳至少有一則留言的頁面。 (可選)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]