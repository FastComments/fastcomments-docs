## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tag | string | chemin | Oui |  |
| tenantId | string | requête | Non |  |

## Réponse

Renvoie: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteHashTag'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (optionnel)
let deleteHashTagRequestBody = DeleteHashTagRequestBody(tenantId: "tenantId_example") // DeleteHashTagRequestBody |  (optionnel)

DefaultAPI.deleteHashTag(tag: tag, tenantId: tenantId, deleteHashTagRequestBody: deleteHashTagRequestBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]