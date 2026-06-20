## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tag | string | path | 예 |  |
| tenantId | string | query | 아니요 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## 예제

[inline-code-attrs-start title = 'deleteHashTag 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 으로 보고해 주세요
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (선택 사항)
let deleteHashTagRequestBody = DeleteHashTagRequestBody(tenantId: "tenantId_example") // DeleteHashTagRequestBody |  (선택 사항)

DefaultAPI.deleteHashTag(tag: tag, tenantId: tenantId, deleteHashTagRequestBody: deleteHashTagRequestBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]