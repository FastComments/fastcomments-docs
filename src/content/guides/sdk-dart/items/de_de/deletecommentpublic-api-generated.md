## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| broadcastId | string | query | Ja |  |
| editKey | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: `PublicAPIDeleteCommentResponse`

## Beispiel

[inline-code-attrs-start title = 'deleteCommentPublic Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteCommentPublic(tenantId, commentId, broadcastId, DeleteCommentPublicOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteCommentPublic: $e\n');
}
[inline-code-end]