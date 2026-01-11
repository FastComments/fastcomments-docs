req
tenantId
afterId

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| afterId | string | query | Nee |  |
| limit | integer | query | Nee |  |
| tags | array | query | Nee |  |

## Response

Geeft terug: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPosts200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPosts Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (optioneel)
let limit = 987 // Int |  (optioneel)
let tags = ["inner_example"] // [String] |  (optioneel)

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

---