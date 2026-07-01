## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## 응답

반환: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCountResponse.swift)

## 예시

[inline-code-attrs-start title = 'getNotificationCount 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let userId = "userId_example" // String |  (옵션)
let urlId = "urlId_example" // String |  (옵션)
let fromCommentId = "fromCommentId_example" // String |  (옵션)
let viewed = true // Bool |  (옵션)
let type = "type_example" // String |  (옵션)

DefaultAPI.getNotificationCount(tenantId: tenantId, options: DefaultAPI.GetNotificationCountOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type)) { (response, error) in
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