Comentaristas anteriores en la página que NO están actualmente en línea. Ordenados por displayName.
Úselo después de agotar /users/online para mostrar una sección "Members".
Paginación por cursor sobre commenterName: el servidor recorre el índice parcial {tenantId, urlId, commenterName}
desde afterName hacia adelante mediante $gt, sin coste de $skip.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí | Identificador de la URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas. |

## Respuesta

Devuelve: `PageUsersOfflineResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identificador de la URL de la página (limpiado en el servidor).
final afterName = afterName_example; // String | Cursor: pase nextAfterName de la respuesta anterior.
final afterUserId = afterUserId_example; // String | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]