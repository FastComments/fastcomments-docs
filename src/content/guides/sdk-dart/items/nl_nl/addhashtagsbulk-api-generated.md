## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |

## Respons

Retourneert: `BulkCreateHashTagsResponse`

## Voorbeeld

[inline-code-attrs-start title = 'addHashTagsBulk Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment hieronder om prefix (bijv. Bearer) in te stellen voor API-sleutel, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final bulkCreateHashTagsBody = BulkCreateHashTagsBody(); // BulkCreateHashTagsBody | 

try {
    final result = api_instance.addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addHashTagsBulk: $e\n');
}
[inline-code-end]