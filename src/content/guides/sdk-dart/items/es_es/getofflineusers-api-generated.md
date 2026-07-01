Past commenters on the page who are NOT currently online. Sorted by displayName.  
Comentadores anteriores en la página que NO están actualmente en línea. Ordenados por displayName.  

Use this after exhausting /users/online to render a "Members" section.  
Utilice esto después de agotar /users/online para renderizar una sección "Members".  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
Paginación por cursor en commenterName: el servidor recorre la parte parcial {tenantId, urlId, commenterName}  

index from afterName forward via $gt, no $skip cost.  
índice desde afterName hacia adelante mediante $gt, sin costo $skip.  

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador de URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pasar nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pasar nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas. |

## Response

Devuelve: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'Ejemplo getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identificador de URL de la página (limpiado en el servidor).
final afterName = afterName_example; // String | Cursor: pasar nextAfterName de la respuesta anterior.
final afterUserId = afterUserId_example; // String | Desempate de cursor: pasar nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]