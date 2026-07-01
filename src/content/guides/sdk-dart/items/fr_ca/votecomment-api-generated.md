## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|-------------|--------|-------------|
| tenantId | string | path | Oui |  |
| commentId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| broadcastId | string | query | Oui |  |
| sessionId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne : `VoteResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple voteComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final urlId = urlId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final voteBodyParams = VoteBodyParams(); // VoteBodyParams | 
final sessionId = sessionId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, VoteCommentOptions(sessionId: sessionId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->voteComment: $e\n');
}
[inline-code-end]