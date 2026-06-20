req
tenantId
afterId

## Parametri

| Nome | Type | Location | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |

## Risposta

Restituisce: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getFeedPosts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalare tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (opzionale)
let limit = 987 // Int |  (opzionale)
let tags = ["inner_example"] // [String] |  (opzionale)

DefaultAPI.getFeedPosts(tenantId: tenantId, afterId: afterId, limit: limit, tags: tags) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]