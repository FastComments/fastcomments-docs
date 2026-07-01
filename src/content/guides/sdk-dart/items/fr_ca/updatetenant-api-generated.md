## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Retourne : `APIEmptyResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de updateTenant'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommenter ci‑dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantBody = UpdateTenantBody(); // UpdateTenantBody | 

try {
    final result = api_instance.updateTenant(tenantId, id, updateTenantBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenant: $e\n');
}
[inline-code-end]