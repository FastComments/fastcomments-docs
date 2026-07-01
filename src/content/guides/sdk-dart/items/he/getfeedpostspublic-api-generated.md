req
tenantId
afterId

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| afterId | string | query | לא |  |
| limit | integer | query | לא |  |
| tags | array | query | לא |  |
| sso | string | query | לא |  |
| isCrawler | boolean | query | לא |  |
| includeUserInfo | boolean | query | לא |  |

## תגובה

מחזיר: `PublicFeedPostsResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמה getFeedPostsPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final afterId = afterId_example; // String | 
final limit = 56; // int | 
final tags = []; // List<String> | 
final sso = sso_example; // String | 
final isCrawler = true; // bool | 
final includeUserInfo = true; // bool | 

try {
    final result = api_instance.getFeedPostsPublic(tenantId, GetFeedPostsPublicOptions(afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getFeedPostsPublic: $e\n');
}
[inline-code-end]

---