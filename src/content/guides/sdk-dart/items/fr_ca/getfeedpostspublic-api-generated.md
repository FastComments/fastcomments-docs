req
tenantId
afterId

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | path | Oui |  |
| afterId | string | query | Non |  |
| limit | integer | query | Non |  |
| tags | array | query | Non |  |
| sso | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeUserInfo | boolean | query | Non |  |

## Réponse

Retourne : `PublicFeedPostsResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getFeedPostsPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
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