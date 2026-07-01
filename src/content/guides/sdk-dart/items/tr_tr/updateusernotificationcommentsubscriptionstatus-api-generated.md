Enable or disable notifications for a specific comment.

## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| notificationId | string | path | Yes |  |
| optedInOrOut | string | path | Yes |  |
| commentId | string | query | Yes |  |
| sso | string | query | No |  |

## Yanıt

Döndürür: `UpdateUserNotificationCommentSubscriptionStatusResponse`

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
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