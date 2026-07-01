## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postIds | array | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: `UserReactsResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getUserReactsPublic Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
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