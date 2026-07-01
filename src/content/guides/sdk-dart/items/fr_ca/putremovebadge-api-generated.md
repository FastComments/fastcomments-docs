## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| badgeId | string | query | Oui |  |
| userId | string | query | Non |  |
| commentId | string | query | Non |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : `RemoveUserBadgeResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple putRemoveBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final badgeId = badgeId_example; // String | 
final userId = userId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putRemoveBadge(tenantId, badgeId, PutRemoveBadgeOptions(userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putRemoveBadge: $e\n');
}
[inline-code-end]