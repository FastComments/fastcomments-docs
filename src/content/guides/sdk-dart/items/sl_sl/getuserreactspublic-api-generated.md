## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| postIds | array | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: `UserReactsResponse`

## Primer

[inline-code-attrs-start title = 'getUserReactsPublic Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postIds = []; // List<String> | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserReactsPublic(tenantId, GetUserReactsPublicOptions(postIds: postIds, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserReactsPublic: $e\n');
}
[inline-code-end]