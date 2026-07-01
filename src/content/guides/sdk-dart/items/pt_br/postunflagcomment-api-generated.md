## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| commentId | string | path | Sim |  |
| broadcastId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Returns: `APIEmptyResponse`

## Exemplo

[inline-code-attrs-start title = 'postUnFlagComment Exemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postUnFlagComment(tenantId, commentId, PostUnFlagCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postUnFlagComment: $e\n');
}
[inline-code-end]