req
tenantId
afterId

## Parametri

| Ime | Tip | Lokacija | Potrebno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| afterId | string | query | Ne |  |
| limit | integer | query | Ne |  |
| tags | array | query | Ne |  |

## Odgovor

Vraća: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer getFeedPosts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (opcionalno)
let limit = 987 // Int |  (opcionalno)
let tags = ["inner_example"] // [String] |  (opcionalno)

DefaultAPI.getFeedPosts(tenantId: tenantId, options: DefaultAPI.GetFeedPostsOptions(afterId: afterId, limit: limit, tags: tags)) { (response, error) in
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