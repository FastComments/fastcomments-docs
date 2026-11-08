## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| afterId | string | query | नहीं |  |
| afterCreatedAt | integer | query | नहीं |  |
| unreadOnly | boolean | query | नहीं |  |
| dmOnly | boolean | query | नहीं |  |
| noDm | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `ResetUserNotificationsResponse`

## उदाहरण

[inline-code-attrs-start title = 'resetUserNotifications उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
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
    print('Exception when calling PublicApi->resetUserNotifications: $e\n');
}
[inline-code-end]