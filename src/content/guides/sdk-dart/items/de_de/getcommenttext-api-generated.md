## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| editKey | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: `PublicAPIGetCommentTextResponse`

## Beispiel

[inline-code-attrs-start title = 'getCommentText Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCommentText(tenantId, commentId, GetCommentTextOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentText: $e\n');
}
[inline-code-end]