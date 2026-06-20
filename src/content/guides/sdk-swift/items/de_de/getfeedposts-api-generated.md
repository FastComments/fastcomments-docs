req
tenantId
afterId

## Parameter

| Name | Type | Location | Erforderlich | Beschreibung |
|------|------|----------|--------------|--------------|
| tenantId | string | query | Ja |  |
| afterId | string | query | Nein |  |
| limit | integer | query | Nein |  |
| tags | array | query | Nein |  |

## Antwort

Gibt zurück: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getFeedPosts Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie diese bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (optional)
let limit = 987 // Int |  (optional)
let tags = ["inner_example"] // [String] |  (optional)

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