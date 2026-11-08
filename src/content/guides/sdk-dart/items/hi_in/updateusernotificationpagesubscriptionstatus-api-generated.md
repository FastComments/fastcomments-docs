किसी पेज के लिए नोटिफिकेशन सक्षम या अक्षम करें। जब उपयोगकर्ता किसी पेज के सब्सक्राइबर होते हैं, तो नई रूट कमेंट्स के लिए नोटिफिकेशन बनाई जाती हैं, और साथ ही

## पैरामीटर्स

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | हाँ |  |
| url | string | query | हाँ |  |
| pageTitle | string | query | हाँ |  |
| subscribedOrUnsubscribed | string | path | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `UpdateUserNotificationPageSubscriptionStatusResponse`

## उदाहरण

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
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