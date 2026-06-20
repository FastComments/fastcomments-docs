## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| broadcastId | string | query | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ChangeCommentPinStatusResponse.swift)

## 예제

[inline-code-attrs-start title = 'unPinComment 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 을 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // 문자열 | 
let commentId = "commentId_example" // 문자열 | 
let broadcastId = "broadcastId_example" // 문자열 | 
let sso = "sso_example" // 문자열 |  (선택 사항)

PublicAPI.unPinComment(tenantId: tenantId, commentId: commentId, broadcastId: broadcastId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]