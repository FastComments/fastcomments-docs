## Parámetros

| Name | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opcional)
let badgeId = "badgeId_example" // String |  (opcional)
let type = 987 // Double |  (opcional)
let displayedOnComments = true // Bool |  (opcional)
let limit = 987 // Double |  (opcional)
let skip = 987 // Double |  (opcional)

DefaultAPI.getUserBadges(tenantId: tenantId, userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip) { (response, error) in
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