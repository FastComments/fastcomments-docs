## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| spam | boolean | query | Nej |  |
| permNotSpam | boolean | query | Nej |  |
| broadcastId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: `APIEmptyResponse`

## Eksempel

[inline-code-attrs-start title = 'postSetCommentSpamStatus Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final spam = true; // bool | 
final permNotSpam = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentSpamStatus(tenantId, commentId, PostSetCommentSpamStatusOptions(spam: spam, permNotSpam: permNotSpam, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentSpamStatus: $e\n');
}
[inline-code-end]