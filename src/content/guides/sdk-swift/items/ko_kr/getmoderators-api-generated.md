## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetModeratorsResponse.swift)

## 예제

[inline-code-attrs-start title = 'getModerators 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플들은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 에서 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let skip = 987 // Double |  (선택 사항)

DefaultAPI.getModerators(tenantId: tenantId, skip: skip) { (response, error) in
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