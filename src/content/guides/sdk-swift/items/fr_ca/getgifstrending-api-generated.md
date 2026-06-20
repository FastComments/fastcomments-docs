## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Réponse

Renvoie : [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsTrendingResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getGifsTrending'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let locale = "locale_example" // String |  (facultatif)
let rating = "rating_example" // String |  (facultatif)
let page = 987 // Double |  (facultatif)

PublicAPI.getGifsTrending(tenantId: tenantId, locale: locale, rating: rating, page: page) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]