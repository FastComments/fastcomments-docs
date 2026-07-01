## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | put | Da |  |
| commentId | string | put | Da |  |
| broadcastId | string | upit | Da |  |
| editKey | string | upit | Ne |  |
| sso | string | upit | Ne |  |

## Odgovor

Vraća: `PublicAPIDeleteCommentResponse`

## Primer

[inline-code-attrs-start title = 'deleteCommentPublic Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
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