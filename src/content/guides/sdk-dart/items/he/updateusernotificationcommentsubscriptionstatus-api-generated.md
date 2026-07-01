הפעל או בטל התראות לתגובה ספציפית.

## Parameters

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| notificationId | string | path | כן |  |
| optedInOrOut | string | path | כן |  |
| commentId | string | query | כן |  |
| sso | string | query | לא |  |

## Response

מחזירה: `UpdateUserNotificationCommentSubscriptionStatusResponse`

## Example

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus דוגמה'; type = ''; isFunctional = false; inline-code-attrs-end]
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