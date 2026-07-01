## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|------------|--------------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: `PostRemoveCommentApiResponse`

## Esempio

[inline-code-attrs-start title = 'postRemoveComment Esempio'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postRemoveComment(tenantId, commentId, PostRemoveCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postRemoveComment: $e\n');
}
[inline-code-end]