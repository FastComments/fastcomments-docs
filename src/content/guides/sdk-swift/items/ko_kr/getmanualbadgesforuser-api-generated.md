## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | 아니요 |  |
| commentId | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserManualBadgesResponse.swift)

## 예제

[inline-code-attrs-start title = 'getManualBadgesForUser 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let badgesUserId = "badgesUserId_example" // String |  (선택 사항)
let commentId = "commentId_example" // String |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

ModerationAPI.getManualBadgesForUser(badgesUserId: badgesUserId, commentId: commentId, sso: sso) { (response, error) in
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