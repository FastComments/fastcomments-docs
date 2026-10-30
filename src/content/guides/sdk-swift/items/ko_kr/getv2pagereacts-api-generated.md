## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | 설정하면 각 react id에 대한 사용자(아바타와 사용자 이름)를 반환합니다. \"public\"만 허용되며, 이는 비공개 프로필을 제외합니다. |

## Response

반환: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Example

[inline-code-attrs-start title = 'getV2PageReacts 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (옵션)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | 설정하면 각 react id에 대한 사용자(아바타와 사용자 이름)를 반환합니다. \"public\"만 허용되며, 이는 비공개 프로필을 제외합니다. (옵션)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]