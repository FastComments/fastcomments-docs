## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | 예 |  |
| tenantId | string | query | 아니요 |  |

## 응답

반환: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PatchHashTag200Response.swift)

## 예제

[inline-code-attrs-start title = 'patchHashTag 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new으로 보고해 주세요
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (선택 사항)
let updateHashTagBody = UpdateHashTagBody(tenantId: "tenantId_example", url: "url_example", tag: "tag_example") // UpdateHashTagBody |  (선택 사항)

DefaultAPI.patchHashTag(tag: tag, tenantId: tenantId, updateHashTagBody: updateHashTagBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]