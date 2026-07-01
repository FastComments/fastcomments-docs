## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : `APIEmptyResponse`

## Exemple

[inline-code-attrs-start title = 'postRestoreDeletedComment Exemple'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postRestoreDeletedComment(tenantId, commentId, PostRestoreDeletedCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postRestoreDeletedComment: $e\n');
}
[inline-code-end]