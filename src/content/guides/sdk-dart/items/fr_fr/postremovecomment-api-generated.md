## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : `PostRemoveCommentApiResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple postRemoveComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postRemoveComment(tenantId, commentId, PostRemoveCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postRemoveComment: $e\n');
}
[inline-code-end]

---