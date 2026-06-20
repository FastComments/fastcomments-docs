---
## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| largeInternalURLSanitized | string | query | Oui |  |

## Réponse

Renvoie: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GifGetLargeResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getGifLarge'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let largeInternalURLSanitized = "largeInternalURLSanitized_example" // String | 

PublicAPI.getGifLarge(tenantId: tenantId, largeInternalURLSanitized: largeInternalURLSanitized) { (response, error) in
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