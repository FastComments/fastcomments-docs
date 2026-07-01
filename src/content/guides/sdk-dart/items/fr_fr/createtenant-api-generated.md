## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |

## Réponse

Returns: `CreateTenantResponse`

## Exemple

[inline-code-attrs-start title = 'createTenant Exemple'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantBody = CreateTenantBody(); // CreateTenantBody | 

try {
    final result = api_instance.createTenant(tenantId, createTenantBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenant: $e\n');
}
[inline-code-end]