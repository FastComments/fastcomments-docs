req
tenantId
afterId

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| afterId | string | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| tags | array | query | 아니오 |  |
| sso | string | query | 아니오 |  |
| isCrawler | boolean | query | 아니오 |  |
| includeUserInfo | boolean | query | 아니오 |  |

## 응답

반환: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## 예제

[inline-code-attrs-start title = 'getFeedPostsPublic 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제 발생 시, http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (옵션)
let limit = 987 // Int |  (옵션)
let tags = ["inner_example"] // [String] |  (옵션)
let sso = "sso_example" // String |  (옵션)
let isCrawler = true // Bool |  (옵션)
let includeUserInfo = true // Bool |  (옵션)

PublicAPI.getFeedPostsPublic(tenantId: tenantId, options: PublicAPI.GetFeedPostsPublicOptions(afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo)) { (response, error) in
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