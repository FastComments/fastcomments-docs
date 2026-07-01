## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| broadcastId | string | query | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: `ChangeCommentPinStatusResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo pinComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.pinComment(tenantId, commentId, broadcastId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->pinComment: $e\n');
}
[inline-code-end]

---