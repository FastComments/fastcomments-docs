## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentIds | string | query | हाँ | कमेंट आईडी की कॉमा से अलग की गई सूची। |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `CheckBlockedCommentsResponse`

## उदाहरण

[inline-code-attrs-start title = 'checkedCommentsForBlocked उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | कमेंट आईडी की कॉमा से अलग की गई सूची।
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]