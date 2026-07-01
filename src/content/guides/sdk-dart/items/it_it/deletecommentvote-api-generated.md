## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| commentId | string | path | Sì |  |
| voteId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| broadcastId | string | query | Sì |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: `VoteDeleteResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio deleteCommentVote'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // Stringa | 
final commentId = commentId_example; // Stringa | 
final voteId = voteId_example; // Stringa | 
final urlId = urlId_example; // Stringa | 
final broadcastId = broadcastId_example; // Stringa | 
final editKey = editKey_example; // Stringa | 
final sso = sso_example; // Stringa | 

try {
    final result = api_instance.deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, DeleteCommentVoteOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteCommentVote: $e\n');
}
[inline-code-end]