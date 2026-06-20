req
tenantId
afterId

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |
| sso | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeUserInfo | boolean | query | No |  |

## Risposta

Restituisce: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getFeedPostsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (opzionale)
let limit = 987 // Int |  (opzionale)
let tags = ["inner_example"] // [String] |  (opzionale)
let sso = "sso_example" // String |  (opzionale)
let isCrawler = true // Bool |  (opzionale)
let includeUserInfo = true // Bool |  (opzionale)

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