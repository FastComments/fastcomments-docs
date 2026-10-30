Identifica la credencial en uso: el inquilino al que pertenece y, para tokens OAuth, el usuario que la autorizó.  
Las integraciones usan esto para probar una conexión y etiquetarla.

## Parameters

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Example

[inline-code-attrs-start title = 'Ejemplo getMe'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]