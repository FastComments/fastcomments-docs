## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|--------------|---------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nein |  |
| mentionGroupIds | array | query | Nein |  |
| sso | string | query | Nein |  |
| searchSection | string | query | Nein |  |

## Antwort

Rückgabe: `SearchUsersResult`

## Beispiel

[inline-code-attrs-start title = 'searchUsers Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
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