## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## 예제

[inline-code-attrs-start title = 'getUserInternalProfile 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 예제는 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 에서 보고해 주세요
import FastCommentsSwift

let commentId = "commentId_example" // String |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

ModerationAPI.getUserInternalProfile(commentId: commentId, sso: sso) { (response, error) in
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