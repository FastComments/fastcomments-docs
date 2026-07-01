Visores actualmente en línea de una página: personas cuya sesión websocket está suscrita a la página en este momento.  
Devuelve anonCount + totalCount (suscriptores de toda la sala, incluidos los espectadores anónimos que no enumeramos).

## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí | Identificador de URL de la página (limpiado del lado del servidor). |
| afterName | string | query | No | Cursor: pasar nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pasar nextAfterUserId de la respuesta anterior. Requerido cuando afterName está definido para que los empates de nombre no eliminen entradas. |

## Respuesta

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Las siguientes muestras de código siguen en beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificador de URL de la página (limpiado del lado del servidor).
let afterName = "afterName_example" // String | Cursor: pasar nextAfterName de la respuesta anterior. (opcional)
let afterUserId = "afterUserId_example" // String | Desempate de cursor: pasar nextAfterUserId de la respuesta anterior. Requerido cuando afterName está definido para que los empates de nombre no eliminen entradas. (opcional)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]