## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| postIds | array | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `UserReactsResponse`

## उदाहरण

[inline-code-attrs-start title = 'getUserReactsPublic उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postIds = []; // List<String> | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserReactsPublic(tenantId, GetUserReactsPublicOptions(postIds: postIds, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserReactsPublic: $e\n');
}
[inline-code-end]