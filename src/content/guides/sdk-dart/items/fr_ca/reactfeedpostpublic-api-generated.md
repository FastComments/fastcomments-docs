## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| isUndo | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Renvoie : `ReactFeedPostResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple reactFeedPostPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // Chaîne | 
final postId = postId_example; // Chaîne | 
final reactBodyParams = ReactBodyParams(); // ReactBodyParams | 
final isUndo = true; // booléen | 
final broadcastId = broadcastId_example; // Chaîne | 
final sso = sso_example; // Chaîne | 

try {
    final result = api_instance.reactFeedPostPublic(tenantId, postId, reactBodyParams, ReactFeedPostPublicOptions(isUndo: isUndo, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->reactFeedPostPublic: $e\n');
}
[inline-code-end]