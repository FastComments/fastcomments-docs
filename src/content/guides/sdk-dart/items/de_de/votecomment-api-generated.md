## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|--------------|---------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| broadcastId | string | query | Ja |  |
| sessionId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Response

Rückgabe: `VoteResponse`

## Beispiel

[inline-code-attrs-start title = 'voteComment Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
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