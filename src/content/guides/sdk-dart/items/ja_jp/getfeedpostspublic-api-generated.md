req
tenantId
afterId

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| afterId | string | query | いいえ |  |
| limit | integer | query | いいえ |  |
| tags | array | query | いいえ |  |
| sso | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |
| includeUserInfo | boolean | query | いいえ |  |

## レスポンス

Returns: `PublicFeedPostsResponse`

## 例

[inline-code-attrs-start title = 'getFeedPostsPublic の例'; type = ''; isFunctional = false; inline-code-attrs-end]
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