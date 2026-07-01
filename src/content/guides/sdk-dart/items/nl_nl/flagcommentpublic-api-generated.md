## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| isFlagged | boolean | query | Ja |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: `APIEmptyResponse`

## Voorbeeld

[inline-code-attrs-start title = 'flagCommentPublic Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final isFlagged = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.flagCommentPublic(tenantId, commentId, isFlagged, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->flagCommentPublic: $e\n');
}
[inline-code-end]