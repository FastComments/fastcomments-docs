## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| commentId | string | path | Sí |  |
| voteId | string | path | Sí |  |
| urlId | string | query | Sí |  |
| broadcastId | string | query | Sí |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: `VoteDeleteResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteCommentVote'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final voteId = voteId_example; // String | 
final urlId = urlId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, DeleteCommentVoteOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteCommentVote: $e\n');
}
[inline-code-end]