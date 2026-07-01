## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Resposta

Retorna: `GetTenantManualBadgesResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getManualBadges'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getManualBadges(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getManualBadges: $e\n');
}
[inline-code-end]

---