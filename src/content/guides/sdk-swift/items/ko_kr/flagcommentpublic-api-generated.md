## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | path | 예 |  |
| isFlagged | boolean | query | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## 예제

[inline-code-attrs-start title = 'flagCommentPublic 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 에서 리포트해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let isFlagged = true // Bool | 
let sso = "sso_example" // String |  (선택 사항)

PublicAPI.flagCommentPublic(tenantId: tenantId, commentId: commentId, isFlagged: isFlagged, sso: sso) { (response, error) in
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