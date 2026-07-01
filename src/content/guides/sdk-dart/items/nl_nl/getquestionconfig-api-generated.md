## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Returns: `GetQuestionConfigResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getQuestionConfig Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getQuestionConfig(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionConfig: $e\n');
}
[inline-code-end]