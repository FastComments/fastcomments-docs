## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| skip | integer | query | Non |  |

## Réponse

Retourne : `GetSSOUsersResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getSSOUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommenter ci-dessous pour définir le préfixe (ex. Bearer) de la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 56; // int | 

try {
    final result = api_instance.getSSOUsers(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUsers: $e\n');
}
[inline-code-end]