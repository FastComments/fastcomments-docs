## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Respons

Returns: `CreateTenantUserResponse`

## Voorbeeld

[inline-code-attrs-start title = 'createTenantUser Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ontkommenteer onderstaande om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantUserBody = CreateTenantUserBody(); // CreateTenantUserBody | 

try {
    final result = api_instance.createTenantUser(tenantId, createTenantUserBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenantUser: $e\n');
}
[inline-code-end]