## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne : `ResetUserNotificationsResponse`

## Exemple

[inline-code-attrs-start title = 'resetUserNotificationCount Exemple'; type = ''; isFunctional = false; inline-code-attrs-end]
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