## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|---------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Respons

Retourneert: `APIEmptyResponse`

## Voorbeeld

[inline-code-attrs-start title = 'replaceTenantPackage Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ontkommenteer hieronder om prefix (bijv. Bearer) voor API-sleutel in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final replaceTenantPackageBody = ReplaceTenantPackageBody(); // ReplaceTenantPackageBody | 

try {
    final result = api_instance.replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->replaceTenantPackage: $e\n');
}
[inline-code-end]