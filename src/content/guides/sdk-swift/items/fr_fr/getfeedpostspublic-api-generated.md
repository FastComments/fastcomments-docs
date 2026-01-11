req
tenantId
afterId

## Paramètres

| Nom | Type | Location | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| afterId | string | query | Non |  |
| limit | integer | query | Non |  |
| tags | array | query | Non |  |
| sso | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeUserInfo | boolean | query | Non |  |

## Réponse

Retourne : [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsPublic200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getFeedPostsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (optionnel)
let limit = 987 // Int |  (optionnel)
let tags = ["inner_example"] // [String] |  (optionnel)
let sso = "sso_example" // String |  (optionnel)
let isCrawler = true // Bool |  (optionnel)
let includeUserInfo = true // Bool |  (optionnel)

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