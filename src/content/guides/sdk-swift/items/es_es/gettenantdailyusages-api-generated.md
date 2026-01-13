## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| yearNumber | number | query | No |  |
| monthNumber | number | query | No |  |
| dayNumber | number | query | No |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsages200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'getTenantDailyUsages Ejemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (opcional)
let monthNumber = 987 // Double |  (opcional)
let dayNumber = 987 // Double |  (opcional)
let skip = 987 // Double |  (opcional)

DefaultAPI.getTenantDailyUsages(tenantId: tenantId, yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]