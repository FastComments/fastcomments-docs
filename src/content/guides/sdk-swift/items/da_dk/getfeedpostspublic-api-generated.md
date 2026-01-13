req
tenantId
afterId

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| afterId | string | query | Nej |  |
| limit | integer | query | Nej |  |
| tags | array | query | Nej |  |
| sso | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |
| includeUserInfo | boolean | query | Nej |  |

## Svar

Returnerer: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsPublic200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'getFeedPostsPublic Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis du oplever problemer, indberet venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (valgfri)
let limit = 987 // Int |  (valgfri)
let tags = ["inner_example"] // [String] |  (valgfri)
let sso = "sso_example" // String |  (valgfri)
let isCrawler = true // Bool |  (valgfri)
let includeUserInfo = true // Bool |  (valgfri)

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