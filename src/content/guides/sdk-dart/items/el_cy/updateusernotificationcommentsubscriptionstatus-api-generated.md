Enable or disable notifications for a specific comment.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| notificationId | string | path | Ναι |  |
| optedInOrOut | string | path | Ναι |  |
| commentId | string | query | Ναι |  |
| sso | string | query | Όχι |  |

## Response

Επιστρέφει: `UpdateUserNotificationCommentSubscriptionStatusResponse`

## Example

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
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