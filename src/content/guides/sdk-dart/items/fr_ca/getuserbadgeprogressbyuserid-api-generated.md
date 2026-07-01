## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | path | Yes |  |

## Réponse

Renvoie : `APIGetUserBadgeProgressResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getUserBadgeProgressByUserId'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l’autorisation de clé d’API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Décommentez ci‑dessous pour configurer le préfixe (p. ex. Bearer) pour la clé d’API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressByUserId(tenantId, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressByUserId: $e\n');
}
[inline-code-end]

---