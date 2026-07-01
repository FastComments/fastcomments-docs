## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Svar

Returnerer: `UnblockSuccess`

## Eksempel

[inline-code-attrs-start title = 'unBlockCommentPublic Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final publicBlockFromCommentParams = PublicBlockFromCommentParams(); // PublicBlockFromCommentParams | 
final sso = sso_example; // String | 

try {
    final result = api_instance.unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->unBlockCommentPublic: $e\n');
}
[inline-code-end]