## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| afterId | string | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| sso | string | query | No |  |

## Одговор

Враћа: `ResetUserNotificationsResponse`

## Пример

[inline-code-attrs-start title = 'resetUserNotifications Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
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
    print('Izuzetak prilikom pozivanja PublicApi->resetUserNotifications: $e\n');
}
[inline-code-end]