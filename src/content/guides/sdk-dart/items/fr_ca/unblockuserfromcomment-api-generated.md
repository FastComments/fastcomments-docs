## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| userId | string | query | Non |  |
| anonUserId | string | query | Non |  |

## Response

Renvoie : `UnblockSuccess`

## Example

[inline-code-attrs-start title = 'Exemple de unBlockUserFromComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé d'API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Décommentez ci-dessous pour configurer le préfixe (p. ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final unBlockFromCommentParams = UnBlockFromCommentParams(); // UnBlockFromCommentParams | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, UnBlockUserFromCommentOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->unBlockUserFromComment: $e\n');
}
[inline-code-end]