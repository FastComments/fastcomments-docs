## 參數

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| commentId | string | 路徑 | 是 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AdjustVotesResponse.swift)

## 範例

[inline-code-attrs-start title = 'postAdjustCommentVotes 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式範例仍為測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let adjustCommentVotesParams = AdjustCommentVotesParams(adjustVoteAmount: 123) // AdjustCommentVotesParams | 
let sso = "sso_example" // String |  (optional)

ModerationAPI.postAdjustCommentVotes(commentId: commentId, adjustCommentVotesParams: adjustCommentVotesParams, sso: sso) { (response, error) in
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