## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|--------------|----------|------|
| tenantId | string | path | Tak |  |
| postIds | array | query | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: `FeedPostsStatsResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład getFeedPostsStats'; type = ''; isFunctional = false; inline-code-attrs-end]
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