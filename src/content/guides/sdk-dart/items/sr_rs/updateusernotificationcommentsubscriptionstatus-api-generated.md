Омогућите или онемогућите обавештења за одређени коментар.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| notificationId | string | path | Yes |  |
| optedInOrOut | string | path | Yes |  |
| commentId | string | query | Yes |  |
| sso | string | query | No |  |

## Одговор

Returns: `UpdateUserNotificationCommentSubscriptionStatusResponse`

## Пример

[inline-code-attrs-start title = 'Пример updateUserNotificationCommentSubscriptionStatus'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final notificationId = notificationId_example; // String | 
final optedInOrOut = optedInOrOut_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->updateUserNotificationCommentSubscriptionStatus: $e\n');
}
[inline-code-end]