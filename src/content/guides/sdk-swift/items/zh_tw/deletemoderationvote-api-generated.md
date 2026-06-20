## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| voteId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteDeleteResponse.swift)

## 範例

[inline-code-attrs-start title = 'deleteModerationVote 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍屬測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let voteId = "voteId_example" // String | 
let sso = "sso_example" // String |  (可選)

ModerationAPI.deleteModerationVote(commentId: commentId, voteId: voteId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]