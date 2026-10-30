## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| sso | string | query | Non |  |
| includeUserInfo | string | query | Non | Lorsqu'il est défini, renvoie les utilisateurs pour chaque identifiant de réaction (avatars et noms d'utilisateur). Seul \"public\" est accepté, ce qui omet les profils privés. |

## Réponse

Retourne : [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getV2PageReacts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optionnel)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Lorsqu'il est défini, renvoie les utilisateurs pour chaque identifiant de réaction (avatars et noms d'utilisateur). Seul \"public\" est accepté, ce qui omet les profils privés. (optionnel)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]