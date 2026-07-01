---
Comentadores anteriores na página que NÃO estão atualmente online. Ordenados por displayName.  
Use isso após esgotar /users/online para renderizar uma seção “Members”.  
Paginação por cursor em commenterName: o servidor percorre o parcial {tenantId, urlId, commenterName}  
índice a partir de afterName avançando via $gt, sem custo de $skip.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | Não | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Desempate de cursor: passe nextAfterUserId da resposta anterior. Necessário quando afterName estiver definido para que empates de nomes não removam entradas. |

## Resposta

Retorna: `PageUsersOfflineResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identificador da URL da página (limpo no servidor).
final afterName = afterName_example; // String | Cursor: passe nextAfterName da resposta anterior.
final afterUserId = afterUserId_example; // String | Desempate de cursor: passe nextAfterUserId da resposta anterior. Necessário quando afterName estiver definido para que empates de nomes não removam entradas.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]

---