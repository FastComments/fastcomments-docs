## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Risposta

Restituisce: `ModerationAPIChildCommentsResponse`

## Esempio

[inline-code-attrs-start title = 'postCommentsByIds Esempio'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentsByIdsParams = CommentsByIdsParams(); // CommentsByIdsParams | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postCommentsByIds(tenantId, commentsByIdsParams, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postCommentsByIds: $e\n');
}
[inline-code-end]