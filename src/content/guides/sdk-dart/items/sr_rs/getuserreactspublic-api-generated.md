## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postIds | array | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vraća: `UserReactsResponse`

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