## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|---------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Antwort

Rückgabe: `GetUserNotificationCountResponse`

## Beispiel

[inline-code-attrs-start title = 'getUserNotificationCount Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final sno = sso_example; // String | 

try {
    final result = api_instance.getUserNotificationCount(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserNotificationCount: $e\n');
}
[inline-code-end]