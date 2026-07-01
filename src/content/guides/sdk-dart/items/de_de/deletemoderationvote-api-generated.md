## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| voteId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Antwort

Rückgabe: `VoteDeleteResponse`

## Beispiel

[inline-code-attrs-start title = 'deleteModerationVote Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final voteId = voteId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteModerationVote(tenantId, commentId, voteId, DeleteModerationVoteOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->deleteModerationVote: $e\n');
}
[inline-code-end]