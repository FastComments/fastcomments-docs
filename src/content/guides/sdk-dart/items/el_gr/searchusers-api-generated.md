## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| usernameStartsWith | string | query | Όχι |  |
| mentionGroupIds | array | query | Όχι |  |
| sso | string | query | Όχι |  |
| searchSection | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: `SearchUsersResult`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα searchUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---