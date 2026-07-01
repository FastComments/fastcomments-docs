## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | path | Sí |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Returns: `APIEmptyResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo postUnFlagComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postUnFlagComment(tenantId, commentId, PostUnFlagCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postUnFlagComment: $e\n');
}
[inline-code-end]