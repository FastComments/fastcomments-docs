## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Respons

Retourneert: `APIEmptyResponse`

## Voorbeeld

[inline-code-attrs-start title = 'deleteEmailTemplate Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// verwijder de commentaartekens hieronder om prefix (bijv. Bearer) in te stellen voor API-sleutel, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteEmailTemplate(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteEmailTemplate: $e\n');
}
[inline-code-end]

---