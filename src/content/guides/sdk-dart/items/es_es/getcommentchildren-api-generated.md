## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | path | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: `ModerationAPIChildCommentsResponse`

## Ejemplo

[inline-code-attrs-start title = 'getCommentChildren Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCommentChildren(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getCommentChildren: $e\n');
}
[inline-code-end]

---