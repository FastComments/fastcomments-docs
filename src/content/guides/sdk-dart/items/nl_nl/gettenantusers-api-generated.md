## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nee |  |

## Respons

Returns: `GetTenantUsersResponse`

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getTenantUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API key autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// verwijder de onder commentaar om een prefix (bijv. Bearer) voor API key in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantUsers(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantUsers: $e\n');
}
[inline-code-end]