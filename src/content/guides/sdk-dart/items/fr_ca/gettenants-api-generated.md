## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| meta | string | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Renvoie : `GetTenantsResponse`

## Exemple

[inline-code-attrs-start title = 'getTenants Exemple'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
//décommentez ci‑dessous pour configurer le préfixe (ex. Bearer) de la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final meta = meta_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenants(tenantId, GetTenantsOptions(meta: meta, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenants: $e\n');
}
[inline-code-end]