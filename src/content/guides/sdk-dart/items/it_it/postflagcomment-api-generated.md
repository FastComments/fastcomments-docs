## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | path | Sì |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: `APIEmptyResponse`

## Esempio

[inline-code-attrs-start title = 'postFlagComment Esempio'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postFlagComment(tenantId, commentId, PostFlagCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postFlagComment: $e\n');
}
[inline-code-end]