Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador de URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pasa nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pasa nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas. |

## Respuesta

Returns: `PageUsersOnlineResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identificador de URL de la página (limpiado en el servidor).
final afterName = afterName_example; // String | Cursor: pasa nextAfterName de la respuesta anterior.
final afterUserId = afterUserId_example; // String | Desempate de cursor: pasa nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]

---