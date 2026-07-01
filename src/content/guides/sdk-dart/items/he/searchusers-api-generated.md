## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| usernameStartsWith | string | query | No |  |
| mentionGroupIds | array | query | No |  |
| sso | string | query | No |  |
| searchSection | string | query | No |  |

## תגובה

מחזיר: `SearchUsersResult`

## דוגמה

[inline-code-attrs-start title = 'דוגמת searchUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // מחרוזת | 
final urlId = urlId_example; // מחרוזת | 
final usernameStartsWith = usernameStartsWith_example; // מחרוזת | 
final mentionGroupIds = []; // רשימה<String> | 
final sso = sso_example; // מחרוזת | 
final searchSection = searchSection_example; // מחרוזת | 

try {
    final result = api_instance.searchUsers(tenantId, urlId, SearchUsersOptions(usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->searchUsers: $e\n');
}
[inline-code-end]