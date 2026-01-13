## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| urlId | string | query | 否 |  |
| fromCommentId | string | query | 否 |  |
| viewed | boolean | query | 否 |  |
| type | string | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳值: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotifications200Response.swift)

## 範例

[inline-code-attrs-start title = 'getNotifications 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。如遇任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (選用)
let urlId = "urlId_example" // String |  (選用)
let fromCommentId = "fromCommentId_example" // String |  (選用)
let viewed = true // Bool |  (選用)
let type = "type_example" // String |  (選用)
let skip = 987 // Double |  (選用)

DefaultAPI.getNotifications(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip) { (response, error) in
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