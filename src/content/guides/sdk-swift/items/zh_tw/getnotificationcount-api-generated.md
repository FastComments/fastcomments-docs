## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| urlId | string | query | 否 |  |
| fromCommentId | string | query | 否 |  |
| viewed | boolean | query | 否 |  |
| type | string | query | 否 |  |

## 回應

回傳: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCount200Response.swift)

## 範例

[inline-code-attrs-start title = 'getNotificationCount 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (選用)
let urlId = "urlId_example" // String |  (選用)
let fromCommentId = "fromCommentId_example" // String |  (選用)
let viewed = true // Bool |  (選用)
let type = "type_example" // String |  (選用)

DefaultAPI.getNotificationCount(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---