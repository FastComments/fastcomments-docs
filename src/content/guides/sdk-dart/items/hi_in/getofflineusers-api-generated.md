पेज पर पिछले टिप्पणीकार जो अभी ऑनलाइन नहीं हैं। displayName के अनुसार क्रमबद्ध।
"Members" सेक्शन रेंडर करने के लिए /users/online को समाप्त करने के बाद इसका उपयोग करें।
commenterName पर कर्सर पेजिनेशन: सर्वर partial {tenantId, urlId, commenterName}
इंडेक्स को afterName से $gt के माध्यम से आगे बढ़ाता है, कोई $skip लागत नहीं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें। |

## प्रतिक्रिया

रिटर्न: `PageUsersOfflineResponse`

## उदाहरण

[inline-code-attrs-start title = 'getOfflineUsers उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
final afterName = afterName_example; // String | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें।
final afterUserId = afterUserId_example; // String | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें।

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]