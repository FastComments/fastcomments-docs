페이지에 대한 사용자 지정 리액트 카운트, 선택적으로 리액트한 사용자 포함.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 예 |  |
| includeUserInfo | string | query | 아니요 | "public"은 비공개 프로필 사용자를 생략하고, "include-private"은 포함합니다. |

## 응답

반환값: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageReactsAPIResponse.swift)

## 예제

[inline-code-attrs-start title = 'getPageReacts 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\"은 비공개 프로필 사용자를 생략하고, \"include-private\"은 포함합니다. (optional)

DefaultAPI.getPageReacts(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]