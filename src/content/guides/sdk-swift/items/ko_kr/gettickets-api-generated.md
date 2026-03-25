## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니요 |  |
| state | number | query | 아니요 |  |
| skip | number | query | 아니요 |  |
| limit | number | query | 아니요 |  |

## 응답

반환: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTickets200Response.swift)

## 예제

[inline-code-attrs-start title = 'getTickets 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (선택 사항)
let state = 987 // Double |  (선택 사항)
let skip = 987 // Double |  (선택 사항)
let limit = 987 // Double |  (선택 사항)

DefaultAPI.getTickets(tenantId: tenantId, userId: userId, state: state, skip: skip, limit: limit) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]