## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Resposta

Retorna: `ResetUserNotificationsResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo resetUserNotificationCount'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.resetUserNotificationCount(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->resetUserNotificationCount: $e\n');
}
[inline-code-end]