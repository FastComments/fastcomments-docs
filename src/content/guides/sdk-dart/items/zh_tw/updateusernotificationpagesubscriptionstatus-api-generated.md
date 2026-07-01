Enable or disable notifications for a page. When users are subscribed to a page, notifications are created
for new root comments, and also

## Parameters

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|------|------|------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |
| url | string | query | 是 |  |
| pageTitle | string | query | 是 |  |
| subscribedOrUnsubscribed | string | path | 是 |  |
| sso | string | query | 否 |  |

## Response

返回： `UpdateUserNotificationPageSubscriptionStatusResponse`

## Example

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final url = url_example; // String | 
final pageTitle = pageTitle_example; // String | 
final subscribedOrUnsubscribed = subscribedOrUnsubscribed_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->updateUserNotificationPageSubscriptionStatus: $e\n');
}
[inline-code-end]

---