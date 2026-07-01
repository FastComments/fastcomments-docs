---
## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| urlId | string | query | Não | Usado para determinar se a página atual está inscrita. |
| pageSize | integer | query | Não |  |
| afterId | string | query | Não |  |
| includeContext | boolean | query | Não |  |
| afterCreatedAt | integer | query | Não |  |
| unreadOnly | boolean | query | Não |  |
| dmOnly | boolean | query | Não |  |
| noDm | boolean | query | Não |  |
| includeTranslations | boolean | query | Não |  |
| includeTenantNotifications | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: `GetMyNotificationsResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserNotifications'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final urlId = urlId_example; // String | Usado para determinar se a página atual está inscrita.
final pageSize = 56; // int |
final afterId = afterId_example; // String |
final includeContext = true; // bool |
final afterCreatedAt = 789; // int |
final unreadOnly = true; // bool |
final dmOnly = true; // bool |
final noDm = true; // bool |
final includeTranslations = true; // bool |
final includeTenantNotifications = true; // bool |
final sso = sso_example; // String |

try {
    final result = api_instance.getUserNotifications(tenantId, GetUserNotificationsOptions(urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserNotifications: $e\n');
}
[inline-code-end]

---