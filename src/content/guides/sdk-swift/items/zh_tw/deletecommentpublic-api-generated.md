## 參數

| 名稱 | Type | Location | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| commentId | string | path | 是 |  |
| broadcastId | string | query | 是 |  |
| editKey | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicAPIDeleteCommentResponse.swift)

## 範例

[inline-code-attrs-start title = 'deleteCommentPublic 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let editKey = "editKey_example" // String |  (可選)
let sso = "sso_example" // String |  (可選)

PublicAPI.deleteCommentPublic(tenantId: tenantId, commentId: commentId, broadcastId: broadcastId, editKey: editKey, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]