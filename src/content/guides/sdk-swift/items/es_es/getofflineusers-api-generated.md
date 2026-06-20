Comentaristas anteriores en la página que NO están actualmente en línea. Ordenados por displayName.
Utilice esto después de agotar /users/online para mostrar una sección "Miembros".
Paginación por cursor en commenterName: el servidor recorre el índice parcial {tenantId, urlId, commenterName}
Índice desde afterName hacia adelante mediante $gt; sin costo de $skip.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí | Identificador de URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que empates por nombre no descarten entradas. |

## Respuesta

Devuelve: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor informe vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificador de URL de la página (limpiado en el servidor).
let afterName = "afterName_example" // String | Cursor: pase nextAfterName de la respuesta anterior. (opcional)
let afterUserId = "afterUserId_example" // String | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no descarten entradas. (opcional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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