## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| search | string | query | Oui |  |
| locale | string | query | Non |  |
| rating | string | query | Non |  |
| page | number | query | Non |  |

## Réponse

Renvoie : [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsSearchResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getGifsSearch'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont toujours en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let search = "search_example" // String | 
let locale = "locale_example" // String |  (optionnel)
let rating = "rating_example" // String |  (optionnel)
let page = 987 // Double |  (optionnel)

PublicAPI.getGifsSearch(tenantId: tenantId, search: search, locale: locale, rating: rating, page: page) { (response, error) in
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