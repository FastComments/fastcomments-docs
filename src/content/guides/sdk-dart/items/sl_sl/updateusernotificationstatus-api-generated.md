## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| notificationId | string | path | Da |  |
| newStatus | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: `UpdateUserNotificationStatusResponse`

## Primer

[inline-code-attrs-start title = 'updateUserNotificationStatus Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---