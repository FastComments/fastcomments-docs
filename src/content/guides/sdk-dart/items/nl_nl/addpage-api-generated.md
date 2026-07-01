## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Respons

Retourneert: `AddPageAPIResponse`

## Voorbeeld

[inline-code-attrs-start title = 'addPage Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ontcommentaar onderstaand om prefix in te stellen (bijv. Bearer) voor API-sleutel, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPIPageData = CreateAPIPageData(); // CreateAPIPageData | 

try {
    final result = api_instance.addPage(tenantId, createAPIPageData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addPage: $e\n');
}
[inline-code-end]