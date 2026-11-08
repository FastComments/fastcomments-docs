किसी टेनेंट के लिए बल्क यूज़र जानकारी। दिए गए userIds के लिए, User / SSOUser से डिस्प्ले जानकारी लौटाता है।
कमेंट विजेट द्वारा उन यूज़र्स को समृद्ध करने के लिए उपयोग किया जाता है जो अभी presence इवेंट के माध्यम से दिखाई दिए हैं।
कोई पेज संदर्भ नहीं: गोपनीयता समान रूप से लागू की जाती है (प्राइवेट प्रोफाइल मास्क कर दी जाती हैं)।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| ids | string | query | हाँ | कॉमा से अलग किए गए userIds। |

## प्रतिक्रिया

लौटाता है: `PageUsersInfoResponse`

## उदाहरण

[inline-code-attrs-start title = 'getUsersInfo उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | कॉमा से अलग किए गए userIds।

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]