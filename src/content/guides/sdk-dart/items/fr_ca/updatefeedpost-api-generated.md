## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Retourne : `APIEmptyResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de updateFeedPost'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // Chaîne | 
final id = id_example; // Chaîne | 
final feedPost = FeedPost(); // FeedPost | 

try {
    final result = api_instance.updateFeedPost(tenantId, id, feedPost);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateFeedPost: $e\n');
}
[inline-code-end]