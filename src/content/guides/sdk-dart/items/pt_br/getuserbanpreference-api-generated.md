## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: `APIModerateGetUserBanPreferencesResponse`

## Exemplo

[inline-code-attrs-start title = 'getUserBanPreference Exemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserBanPreference(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getUserBanPreference: $e\n');
}
[inline-code-end]

---