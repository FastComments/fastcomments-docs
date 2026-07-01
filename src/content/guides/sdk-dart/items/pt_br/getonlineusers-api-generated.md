Visualizadores atualmente online de uma página: pessoas cuja sessão websocket está assinada à página neste momento.  
Retorna anonCount + totalCount (assinantes de toda a sala, incluindo visualizadores anônimos que não enumeramos).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador da URL da página (limpo no lado do servidor). |
| afterName | string | query | No | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | No | Desempate de cursor: passe nextAfterUserId da resposta anterior. Necessário quando afterName está definido para que empates de nome não removam entradas. |

## Response

Retorna: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Exemplo getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identificador da URL da página (limpo no lado do servidor).
final afterName = afterName_example; // String | Cursor: passe nextAfterName da resposta anterior.
final afterUserId = afterUserId_example; // String | Desempate de cursor: passe nextAfterUserId da resposta anterior. Necessário quando afterName está definido para que empates de nome não removam entradas.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]