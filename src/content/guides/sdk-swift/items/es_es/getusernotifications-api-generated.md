## Parámetros

| Nombre | Tipo | Location | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| urlId | string | query | No | Se utiliza para determinar si la página actual está suscrita. |
| pageSize | integer | query | No |  |
| afterId | string | query | No |  |
| includeContext | boolean | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| includeTranslations | boolean | query | No |  |
| includeTenantNotifications | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, repórtelo a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Se utiliza para determinar si la página actual está suscrita. (opcional)
let pageSize = 987 // Int |  (opcional)
let afterId = "afterId_example" // String |  (opcional)
let includeContext = true // Bool |  (opcional)
let afterCreatedAt = 987 // Int64 |  (opcional)
let unreadOnly = true // Bool |  (opcional)
let dmOnly = true // Bool |  (opcional)
let noDm = true // Bool |  (opcional)
let includeTranslations = true // Bool |  (opcional)
let includeTenantNotifications = true // Bool |  (opcional)
let sso = "sso_example" // String |  (opcional)

PublicAPI.getUserNotifications(tenantId: tenantId, urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]