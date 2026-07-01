## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| commentId | string | path | Oui |  |
| voteId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| broadcastId | string | query | Oui |  |
| editKey | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : `VoteDeleteResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de suppression du vote de commentaire'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // Chaîne | 
final commentId = commentId_example; // Chaîne | 
final voteId = voteId_example; // Chaîne | 
final urlId = urlId_example; // Chaîne | 
final broadcastId = broadcastId_example; // Chaîne | 
final editKey = editKey_example; // Chaîne | 
final sso = sso_example; // Chaîne | 

try {
    final result = api_instance.deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, DeleteCommentVoteOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteCommentVote: $e\n');
}
[inline-code-end]