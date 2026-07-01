## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|---------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Respons

Returns: `APIEmptyResponse`

## Voorbeeld

[inline-code-attrs-start title = 'updateTenant Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// verwijder de onder commentaar om prefix (bijv. Bearer) voor API-sleutel in te stellen, indien nodig
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