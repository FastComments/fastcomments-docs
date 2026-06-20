---
## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| yearNumber | number | query | 아니요 |  |
| monthNumber | number | query | 아니요 |  |
| dayNumber | number | query | 아니요 |  |
| skip | number | query | 아니요 |  |

## 응답

반환: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsagesResponse.swift)

## 예제

[inline-code-attrs-start title = 'getTenantDailyUsages 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 여전히 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (선택 사항)
let monthNumber = 987 // Double |  (선택 사항)
let dayNumber = 987 // Double |  (선택 사항)
let skip = 987 // Double |  (선택 사항)

DefaultAPI.getTenantDailyUsages(tenantId: tenantId, yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip) { (response, error) in
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