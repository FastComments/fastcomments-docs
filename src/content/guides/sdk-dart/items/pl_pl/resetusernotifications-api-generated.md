## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| afterId | string | query | Nie |  |
| afterCreatedAt | integer | query | Nie |  |
| unreadOnly | boolean | query | Nie |  |
| dmOnly | boolean | query | Nie |  |
| noDm | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: `ResetUserNotificationsResponse`

## Przykład

[inline-code-attrs-start title = 'resetUserNotifications Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final afterId = afterId_example; // String | 
final afterCreatedAt = 789; // int | 
final unreadOnly = true; // bool | 
final dmOnly = true; // bool | 
final noDm = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.resetUserNotifications(tenantId, ResetUserNotificationsOptions(afterId: afterId, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->resetUserNotifications: $e\n');
}
[inline-code-end]