## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | forespørgsel | Ja |  |
| commentId | string | sti | Ja |  |
| broadcastId | string | forespørgsel | Nej |  |
| sso | string | forespørgsel | Nej |  |

## Response

Returnerer: `AdjustVotesResponse`

## Example

[inline-code-attrs-start title = 'postAdjustCommentVotes Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final adjustCommentVotesParams = AdjustCommentVotesParams(); // AdjustCommentVotesParams | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postAdjustCommentVotes(tenantId, commentId, adjustCommentVotesParams, PostAdjustCommentVotesOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postAdjustCommentVotes: $e\n');
}
[inline-code-end]