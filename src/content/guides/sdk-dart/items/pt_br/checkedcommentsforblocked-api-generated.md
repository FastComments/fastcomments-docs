## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| commentIds | string | query | Sim | Uma lista separada por vírgulas de IDs de comentários. |
| sso | string | query | Não |  |

## Resposta

Retorna: `CheckBlockedCommentsResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo de checkedCommentsForBlocked'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Uma lista separada por vírgulas de IDs de comentários.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]

---