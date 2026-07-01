## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Réponse

Renvoie : `APICreateUserBadgeResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple createUserBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommenter ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createUserBadgeParams = CreateUserBadgeParams(); // CreateUserBadgeParams | 

try {
    final result = api_instance.createUserBadge(tenantId, createUserBadgeParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createUserBadge: $e\n');
}
[inline-code-end]