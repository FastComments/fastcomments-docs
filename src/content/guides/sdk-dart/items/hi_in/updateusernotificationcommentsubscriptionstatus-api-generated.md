किसी विशिष्ट कमेंट के लिए नोटिफिकेशन सक्षम या अक्षम करें।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| notificationId | string | path | हाँ |  |
| optedInOrOut | string | path | हाँ |  |
| commentId | string | query | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `UpdateUserNotificationCommentSubscriptionStatusResponse`

## उदाहरण

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
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