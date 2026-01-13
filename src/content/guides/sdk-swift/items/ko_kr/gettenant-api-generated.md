## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |

## 응답

반환: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenant200Response.swift)

## 예제

[inline-code-attrs-start title = 'getTenant 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 아래 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // 문자열 | 
let id = "id_example" // 문자열 | 

DefaultAPI.getTenant(tenantId: tenantId, id: id) { (response, error) in
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