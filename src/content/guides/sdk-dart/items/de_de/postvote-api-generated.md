## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|--------------|---------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| direction | string | query | Nein |  |
| broadcastId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Returns: `VoteResponse`

## Beispiel

[inline-code-attrs-start title = 'postVote Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final direction = direction_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postVote(tenantId, commentId, PostVoteOptions(direction: direction, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postVote: $e\n');
}
[inline-code-end]