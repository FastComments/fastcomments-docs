為特定評論啟用或停用通知。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| notificationId | string | path | Yes |  |
| optedInOrOut | string | path | Yes |  |
| commentId | string | query | Yes |  |
| sso | string | query | No |  |

## 回應

返回: `UpdateUserNotificationCommentSubscriptionStatusResponse`

## 範例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
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