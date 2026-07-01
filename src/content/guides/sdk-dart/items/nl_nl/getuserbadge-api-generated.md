## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|---------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Respons

Returns: `APIGetUserBadgeResponse`

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getUserBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// verwijder commentaar hieronder om prefix (bijv. Bearer) voor API-sleutel in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUserBadge(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadge: $e\n');
}
[inline-code-end]