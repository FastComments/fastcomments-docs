## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `GetUserNotificationCountResponse`

## Приклад

[inline-code-attrs-start title = 'getUserNotificationCount Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserNotificationCount(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserNotificationCount: $e\n');
}
[inline-code-end]