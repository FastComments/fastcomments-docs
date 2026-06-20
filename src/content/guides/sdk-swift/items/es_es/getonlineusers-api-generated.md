---
Espectadores actualmente en línea de una página: personas cuya sesión websocket está suscrita a la página en este momento.
Devuelve anonCount + totalCount (suscriptores de la sala en general, incluidos espectadores anónimos que no enumeramos).

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí | Identificador de la URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no eliminen entradas. |

## Respuesta

Devuelve: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'getOnlineUsers Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificador de la URL de la página (limpiado en el servidor).
let afterName = "afterName_example" // String | Cursor: pase nextAfterName de la respuesta anterior. (opcional)
let afterUserId = "afterUserId_example" // String | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no eliminen entradas. (opcional)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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