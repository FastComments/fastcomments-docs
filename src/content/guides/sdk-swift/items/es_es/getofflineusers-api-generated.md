---
Comentadores anteriores en la página que NO están actualmente en línea. Ordenados por displayName.  
Utilízalo después de agotar /users/online para renderizar una sección "Members".  
Paginación por cursor en commenterName: el servidor recorre el parcial {tenantId, urlId, commenterName} index desde afterName hacia adelante mediante $gt, sin costo de $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador de URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pasa nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pasa nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas. |

## Response

Devuelve: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'Ejemplo getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
// Los siguientes ejemplos de código están todavía en beta. Para cualquier problema, por favor repórtalo vía http://github.com/OpenAPITools/openapi-generator/issues/new  
import FastCommentsSwift  

let tenantId = "tenantId_example" // String |  
let urlId = "urlId_example" // String | Identificador de URL de la página (limpiado en el servidor).  
let afterName = "afterName_example" // String | Cursor: pasa nextAfterName de la respuesta anterior. (opcional)  
let afterUserId = "afterUserId_example" // String | Desempate de cursor: pasa nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas. (opcional)  

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in  
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