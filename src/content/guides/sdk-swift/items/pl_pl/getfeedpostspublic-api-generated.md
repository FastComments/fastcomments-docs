req
tenantId
afterId

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | ścieżka | Tak |  |
| afterId | string | zapytanie | Nie |  |
| limit | integer | zapytanie | Nie |  |
| tags | array | zapytanie | Nie |  |
| sso | string | zapytanie | Nie |  |
| isCrawler | boolean | zapytanie | Nie |  |
| includeUserInfo | boolean | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getFeedPostsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W razie jakichkolwiek problemów, zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (opcjonalne)
let limit = 987 // Int |  (opcjonalne)
let tags = ["inner_example"] // [String] |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)
let isCrawler = true // Bool |  (opcjonalne)
let includeUserInfo = true // Bool |  (opcjonalne)

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

---