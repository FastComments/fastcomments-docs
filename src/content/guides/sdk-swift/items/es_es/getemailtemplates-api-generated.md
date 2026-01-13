## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | consulta | Sí |  |
| skip | number | consulta | No |  |

## Respuesta

Devuelve: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplates200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getEmailTemplates'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes fragmentos de código aún están en beta. Para cualquier problema, por favor repórtelo en http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let skip = 987 // Double |  (opcional)

DefaultAPI.getEmailTemplates(tenantId: tenantId, skip: skip) { (response, error) in
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