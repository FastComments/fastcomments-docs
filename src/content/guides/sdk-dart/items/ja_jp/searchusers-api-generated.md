## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| usernameStartsWith | string | query | いいえ |  |
| mentionGroupIds | array | query | いいえ |  |
| sso | string | query | いいえ |  |
| searchSection | string | query | いいえ |  |

## レスポンス

戻り値: `SearchUsersResult`

## 例

[inline-code-attrs-start title = 'searchUsers の例'; type = ''; isFunctional = false; inline-code-attrs-end]
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