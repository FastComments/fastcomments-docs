## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

Returns: `ResetUserNotificationsResponse`

## 範例

[inline-code-attrs-start title = 'resetUserNotificationCount 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.resetUserNotificationCount(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->resetUserNotificationCount: $e\n');
}
[inline-code-end]