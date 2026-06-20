## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| banEmail | boolean | query | 아니요 |  |
| banEmailDomain | boolean | query | 아니요 |  |
| banIP | boolean | query | 아니요 |  |
| deleteAllUsersComments | boolean | query | 아니요 |  |
| bannedUntil | string | query | 아니요 |  |
| isShadowBan | boolean | query | 아니요 |  |
| updateId | string | query | 아니요 |  |
| banReason | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## 예제

[inline-code-attrs-start title = 'postBanUserFromComment 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 신고해 주세요
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (선택 사항)
let banEmailDomain = true // Bool |  (선택 사항)
let banIP = true // Bool |  (선택 사항)
let deleteAllUsersComments = true // Bool |  (선택 사항)
let bannedUntil = "bannedUntil_example" // String |  (선택 사항)
let isShadowBan = true // Bool |  (선택 사항)
let updateId = "updateId_example" // String |  (선택 사항)
let banReason = "banReason_example" // String |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

ModerationAPI.postBanUserFromComment(commentId: commentId, banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]