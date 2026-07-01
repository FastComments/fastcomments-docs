## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| broadcastId | string | query | Oui |  |
| sessionId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : `SaveCommentsResponseWithPresence`

## Exemple

[inline-code-attrs-start title = 'Exemple createCommentPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final commentData = CommentData(); // CommentData | 
final sessionId = sessionId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.createCommentPublic(tenantId, urlId, broadcastId, commentData, CreateCommentPublicOptions(sessionId: sessionId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createCommentPublic: $e\n');
}
[inline-code-end]

---