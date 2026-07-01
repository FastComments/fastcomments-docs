## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| notificationId | string | path | Ja |  |
| newStatus | string | path | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: `UpdateUserNotificationStatusResponse`

## Eksempel

[inline-code-attrs-start title = 'updateUserNotificationStatus Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
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