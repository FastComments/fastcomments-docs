## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| state | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |
| limit | number | query | 아니오 |  |

## Response

Returns: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicketsResponse.swift)

## Example

[inline-code-attrs-start title = 'getTickets 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 주소로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (선택 사항)
let state = 987 // Double |  (선택 사항)
let skip = 987 // Double |  (선택 사항)
let limit = 987 // Double |  (선택 사항)

DefaultAPI.getTickets(tenantId: tenantId, options: DefaultAPI.GetTicketsOptions(userId: userId, state: state, skip: skip, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]