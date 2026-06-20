## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| voteId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| broadcastId | string | query | 예 |  |
| editKey | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteDeleteResponse.swift)

## 예제

[inline-code-attrs-start title = 'deleteCommentVote 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 에서 신고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // 문자열 | 
let commentId = "commentId_example" // 문자열 | 
let voteId = "voteId_example" // 문자열 | 
let urlId = "urlId_example" // 문자열 | 
let broadcastId = "broadcastId_example" // 문자열 | 
let editKey = "editKey_example" // 문자열 |  (선택 사항)
let sso = "sso_example" // 문자열 |  (선택 사항)

PublicAPI.deleteCommentVote(tenantId: tenantId, commentId: commentId, voteId: voteId, urlId: urlId, broadcastId: broadcastId, editKey: editKey, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]