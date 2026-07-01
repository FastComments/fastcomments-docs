## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| direction | string | query | Nee |  |
| broadcastId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: `VoteResponse`

## Voorbeeld

[inline-code-attrs-start title = 'postVote Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
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