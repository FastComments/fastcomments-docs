## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|-------------|--------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : `APIEmptyResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple postUnFlagComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // Chaîne | 
final commentId = commentId_example; // Chaîne | 
final broadcastId = broadcastId_example; // Chaîne | 
final sso = sso_example; // Chaîne | 

try {
    final result = api_instance.postUnFlagComment(tenantId, commentId, PostUnFlagCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postUnFlagComment: $e\n');
}
[inline-code-end]

---