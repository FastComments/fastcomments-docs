List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| 名稱 | 型別 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Optional case-insensitive title prefix filter. |
| sortBy | string | query | No | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical). |
| hasComments | boolean | query | No | If true, only return pages with at least one comment. |

## Response

回傳：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
```swift
// 以下程式碼範例仍屬 Beta 版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let cursor = "cursor_example" // String | 不透明的分頁游標，從先前的請求中作為 `nextCursor` 回傳。與相同的 `sortBy` 相關聯。（可選）
let limit = 987 // Int | 1..200，預設 50（可選）
let q = "q_example" // String | 可選的大小寫不敏感的標題前綴過濾。（可選）
let sortBy = PagesSortBy() // PagesSortBy | 排序順序。`updatedAt`（預設，最新優先），`commentCount`（評論數最多優先），或 `title`（字母順序）。（可選）
let hasComments = true // Bool | 若為 true，僅回傳至少有一則評論的頁面。（可選）

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```
[inline-code-end]