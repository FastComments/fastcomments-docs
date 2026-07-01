## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| trustFactor | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Retourne : `SetUserTrustFactorResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple setTrustFactor'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // Chaîne | 
final userId = userId_example; // Chaîne | 
final trustFactor = trustFactor_example; // Chaîne | 
final sso = sso_example; // Chaîne | 

try {
    final result = api_instance.setTrustFactor(tenantId, SetTrustFactorOptions(userId: userId, trustFactor: trustFactor, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->setTrustFactor: $e\n');
}
[inline-code-end]