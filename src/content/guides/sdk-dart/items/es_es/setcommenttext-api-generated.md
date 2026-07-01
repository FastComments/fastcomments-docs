## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | path | Sí |  |
| commentId | string | path | Sí |  |
| broadcastId | string | query | Sí |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: `PublicAPISetCommentTextResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de setCommentText'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final commentTextUpdateRequest = CommentTextUpdateRequest(); // CommentTextUpdateRequest | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, SetCommentTextOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->setCommentText: $e\n');
}
[inline-code-end]