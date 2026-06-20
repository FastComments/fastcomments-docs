## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | 경로 | 예 |  |
| sso | string | 쿼리 | 아니오 |  |

## 응답

반환: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBannedUsersFromCommentResponse.swift)

## 예제

[inline-code-attrs-start title = 'getBanUsersFromComment 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (선택 사항)

ModerationAPI.getBanUsersFromComment(commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]