## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Réponse

Renvoie : `GetTenantPackagesResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getTenantPackages'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l\'autorisation de clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommenter ci‑dessous pour configurer le préfixe (p. ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantPackages(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception lors de l\'appel de DefaultApi->getTenantPackages: $e\n');
}
[inline-code-end]

---