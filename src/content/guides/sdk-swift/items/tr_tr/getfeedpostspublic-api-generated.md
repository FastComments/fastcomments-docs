req
tenantId
afterId

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| afterId | string | query | Hayır |  |
| limit | integer | query | Hayır |  |
| tags | array | query | Hayır |  |
| sso | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeUserInfo | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsPublic200Response.swift)

## Örnek

[inline-code-attrs-start title = 'getFeedPostsPublic Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (optional)
let limit = 987 // Int |  (optional)
let tags = ["inner_example"] // [String] |  (optional)
let sso = "sso_example" // String |  (optional)
let isCrawler = true // Bool |  (optional)
let includeUserInfo = true // Bool |  (optional)

PublicAPI.getFeedPostsPublic(tenantId: tenantId, afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]