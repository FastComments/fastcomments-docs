---
## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postIds | array | query | Yes |  |
| sso | string | query | No |  |

## Svar

Returnerer: `FeedPostsStatsResponse`

## Eksempel

[inline-code-attrs-start title = 'getFeedPostsStats Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postIds = []; // List<String> | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getFeedPostsStats(tenantId, postIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getFeedPostsStats: $e\n');
}
[inline-code-end]

---