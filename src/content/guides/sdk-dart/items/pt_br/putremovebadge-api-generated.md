## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|------------|-----------|
| tenantId | string | query | Sim |  |
| badgeId | string | query | Sim |  |
| userId | string | query | Não |  |
| commentId | string | query | Não |  |
| broadcastId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: `RemoveUserBadgeResponse`

## Exemplo

[inline-code-attrs-start title = 'putRemoveBadge Exemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final badgeId = badgeId_example; // String | 
final userId = userId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putRemoveBadge(tenantId, badgeId, PutRemoveBadgeOptions(userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putRemoveBadge: $e\n');
}
[inline-code-end]