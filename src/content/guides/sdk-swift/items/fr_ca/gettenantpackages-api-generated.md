## Paramètres

| Name | Type | Location | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| skip | number | query | Non |  |

## Réponse

Retourne : [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantPackages200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTenantPackages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let skip = 987 // Double |  (facultatif)

DefaultAPI.getTenantPackages(tenantId: tenantId, skip: skip) { (response, error) in
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