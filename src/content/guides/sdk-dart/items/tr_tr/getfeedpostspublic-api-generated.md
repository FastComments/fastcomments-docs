req
tenantId
afterId

## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| afterId | string | query | Hayır |  |
| limit | integer | query | Hayır |  |
| tags | array | query | Hayır |  |
| sso | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeUserInfo | boolean | query | Hayır |  |

## Yanıt

Döndürür: `PublicFeedPostsResponse`

## Örnek

[inline-code-attrs-start title = 'getFeedPostsPublic Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
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