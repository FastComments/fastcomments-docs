## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 아니요 |  |

## 응답

반환: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddHashTag200Response.swift)

## 예제

[inline-code-attrs-start title = 'addHashTag 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제는 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 신고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String |  (선택 사항)
let createHashTagBody = CreateHashTagBody(tenantId: "tenantId_example", tag: "tag_example", url: "url_example") // CreateHashTagBody |  (선택 사항)

DefaultAPI.addHashTag(tenantId: tenantId, createHashTagBody: createHashTagBody) { (response, error) in
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