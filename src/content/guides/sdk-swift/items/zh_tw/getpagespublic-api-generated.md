列出租戶的頁面。由 FChat 桌面用戶端用來填入其房間清單。
每個頁面的已解析自訂設定中必須將 `enableFChat` 設為 true。
需要 SSO 的頁面會根據請求使用者的群組存取權限進行篩選。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 由先前請求以 `nextCursor` 回傳的不透明分頁游標。與相同的 `sortBy` 綁定。 |
| limit | integer | query | 否 | 1..200,預設 50 |
| q | string | query | 否 | 選用的不區分大小寫標題前綴篩選器。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`(預設,最新在前)、`commentCount`(留言數最多在前)或 `title`(依字母順序)。 |
| hasComments | boolean | query | 否 | 若為 true,僅回傳至少有一則留言的頁面。 |

## 回應

回傳:[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 由先前請求以 `nextCursor` 回傳的不透明分頁游標。與相同的 `sortBy` 綁定。 (optional)
let limit = 987 // Int | 1..200,預設 50 (optional)
let q = "q_example" // String | 選用的不區分大小寫標題前綴篩選器。 (optional)
let sortBy = PagesSortBy() // PagesSortBy | 排序方式。`updatedAt`(預設,最新在前)、`commentCount`(留言數最多在前)或 `title`(依字母順序)。 (optional)
let hasComments = true // Bool | 若為 true,僅回傳至少有一則留言的頁面。 (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]