req
tenantId
afterId

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | putanja | Da |  |
| afterId | string | upit | Ne |  |
| limit | integer | upit | Ne |  |
| tags | array | upit | Ne |  |
| sso | string | upit | Ne |  |
| isCrawler | boolean | upit | Ne |  |
| includeUserInfo | boolean | upit | Ne |  |

## Odgovor

Vraća: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## Primer

[inline-code-attrs-start title = 'getFeedPostsPublic Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (opciono)
let limit = 987 // Int |  (opciono)
let tags = ["inner_example"] // [String] |  (opciono)
let sso = "sso_example" // String |  (opciono)
let isCrawler = true // Bool |  (opciono)
let includeUserInfo = true // Bool |  (opciono)

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