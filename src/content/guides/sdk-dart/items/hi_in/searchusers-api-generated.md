## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| usernameStartsWith | string | query | नहीं |  |
| mentionGroupIds | array | query | नहीं |  |
| sso | string | query | नहीं |  |
| searchSection | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: `SearchUsersResult`

## उदाहरण

[inline-code-attrs-start title = 'searchUsers उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final usernameStartsWith = usernameStartsWith_example; // String | 
final mentionGroupIds = []; // List<String> | 
final sso = sso_example; // String | 
final searchSection = searchSection_example; // String | 

try {
    final result = api_instance.searchUsers(tenantId, urlId, SearchUsersOptions(usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->searchUsers: $e\n');
}
[inline-code-end]