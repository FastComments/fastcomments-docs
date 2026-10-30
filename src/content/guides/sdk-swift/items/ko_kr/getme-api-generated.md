Identifies the credential in use: the tenant it belongs to and, for OAuth tokens, the user who authorized it.  
사용 중인 자격 증명을 식별합니다: 해당 자격 증명이 속한 테넌트와 OAuth 토큰인 경우 이를 승인한 사용자를 나타냅니다.  

Integrations use this to test a connection and label it.  
통합에서는 이를 사용하여 연결을 테스트하고 라벨을 지정합니다.  

## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## Response

반환: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Example

[inline-code-attrs-start title = 'getMe 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
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