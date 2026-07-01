## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Renvoie : `CreateFeedPostResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de updateFeedPostPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // Chaîne | 
final postId = postId_example; // Chaîne | 
final updateFeedPostParams = UpdateFeedPostParams(); // UpdateFeedPostParams | 
final broadcastId = broadcastId_example; // Chaîne | 
final sso = sso_example; // Chaîne | 

try {
    final result = api_instance.updateFeedPostPublic(tenantId, postId, updateFeedPostParams, UpdateFeedPostPublicOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception lors de l\'appel PublicApi->updateFeedPostPublic: $e\n');
}
[inline-code-end]