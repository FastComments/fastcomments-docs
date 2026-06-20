## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 是 |  |
| direction | string | query | 是 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |

## 回應

回傳：[`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## 範例

[inline-code-attrs-start title = 'createVote 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let direction = "direction_example" // String | 
let userId = "userId_example" // String |  (可選)
let anonUserId = "anonUserId_example" // String |  (可選)

DefaultAPI.createVote(tenantId: tenantId, commentId: commentId, direction: direction, userId: userId, anonUserId: anonUserId) { (response, error) in
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