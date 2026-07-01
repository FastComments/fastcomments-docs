## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| dir | integer | query | Ja |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: `GetCommentVoteUserNamesSuccessResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getCommentVoteUserNames Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final dir = 56; // int | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCommentVoteUserNames(tenantId, commentId, dir, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentVoteUserNames: $e\n');
}
[inline-code-end]