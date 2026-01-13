## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|--------|------|----------|------------|-------------|
| tenantId | string | query | Sí |  |
| pageSize | integer | query | No |  |
| afterId | string | query | No |  |
| includeContext | boolean | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| includeTranslations | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserNotifications200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, informe vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let pageSize = 987 // Int |  (opcional)
let afterId = "afterId_example" // String |  (opcional)
let includeContext = true // Bool |  (opcional)
let afterCreatedAt = 987 // Int64 |  (opcional)
let unreadOnly = true // Bool |  (opcional)
let dmOnly = true // Bool |  (opcional)
let noDm = true // Bool |  (opcional)
let includeTranslations = true // Bool |  (opcional)
let sso = "sso_example" // String |  (opcional)

PublicAPI.getUserNotifications(tenantId: tenantId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, sso: sso) { (response, error) in
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