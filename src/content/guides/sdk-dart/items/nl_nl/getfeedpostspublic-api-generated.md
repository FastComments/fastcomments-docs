req
tenantId
afterId

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | path | Ja |  |
| afterId | string | query | Nee |  |
| limit | integer | query | Nee |  |
| tags | array | query | Nee |  |
| sso | string | query | Nee |  |
| isCrawler | boolean | query | Nee |  |
| includeUserInfo | boolean | query | Nee |  |

## Respons

Retourneert: `PublicFeedPostsResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPostsPublic Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
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