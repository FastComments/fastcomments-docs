## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| largeInternalURLSanitized | string | query | 예 |  |

## 응답

반환: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GifGetLargeResponse.swift)

## 예제

[inline-code-attrs-start title = 'getGifLarge 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 을 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let largeInternalURLSanitized = "largeInternalURLSanitized_example" // String | 

PublicAPI.getGifLarge(tenantId: tenantId, largeInternalURLSanitized: largeInternalURLSanitized) { (response, error) in
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