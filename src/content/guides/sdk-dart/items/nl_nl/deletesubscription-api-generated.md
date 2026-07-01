## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nee |  |

## Respons

Retourneert: `DeleteSubscriptionAPIResponse`

## Voorbeeld

[inline-code-attrs-start title = 'deleteSubscription Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.deleteSubscription(tenantId, id, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteSubscription: $e\n');
}
[inline-code-end]

---