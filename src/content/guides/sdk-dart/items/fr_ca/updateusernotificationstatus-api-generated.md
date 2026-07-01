## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| notificationId | string | path | Oui |  |
| newStatus | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Returns: `UpdateUserNotificationStatusResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple updateUserNotificationStatus'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final notificationId = notificationId_example; // String | 
final newStatus = newStatus_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->updateUserNotificationStatus: $e\n');
}
[inline-code-end]