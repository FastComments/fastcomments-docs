## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|------------|-------------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| id | string | query | Oui |  |
| sso | string | query | Non |  |

## Response

Retourne : [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReactUsersResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemple getV2PageReactUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let id = "id_example" // String | 
let sso = "sso_example" // String |  (facultatif)

PublicAPI.getV2PageReactUsers(tenantId: tenantId, urlId: urlId, id: id, sso: sso) { (response, error) in
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