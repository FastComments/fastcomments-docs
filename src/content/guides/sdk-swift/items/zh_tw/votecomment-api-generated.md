## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| commentId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| broadcastId | string | query | 是 |  |
| sessionId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## 範例

[inline-code-attrs-start title = 'voteComment 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍處於測試階段。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let voteBodyParams = VoteBodyParams(commenterEmail: "commenterEmail_example", commenterName: "commenterName_example", voteDir: "voteDir_example", url: "url_example") // VoteBodyParams | 
let sessionId = "sessionId_example" // String |  (可選)
let sso = "sso_example" // String |  (可選)

PublicAPI.voteComment(tenantId: tenantId, commentId: commentId, urlId: urlId, broadcastId: broadcastId, voteBodyParams: voteBodyParams, sessionId: sessionId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]