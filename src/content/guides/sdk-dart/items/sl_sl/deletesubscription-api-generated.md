## Parameters

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| userId | string | query | Ne |  |

## Response

Vrne: `DeleteSubscriptionAPIResponse`

## Primer

[inline-code-attrs-start title = 'deleteSubscription Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO: konfiguriraj avtentikacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentiraj spodaj, da nastaviš predpona (npr. Bearer) za API ključ, po potrebi
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