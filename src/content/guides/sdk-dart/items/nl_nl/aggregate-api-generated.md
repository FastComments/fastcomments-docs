Aggregates documenten door ze te groeperen (indien groupBy is opgegeven) en meerdere bewerkingen toe te passen.  
Verschillende bewerkingen (bijv. sum, countDistinct, avg, etc.) worden ondersteund.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| parentTenantId | string | query | Nee |  |
| includeStats | boolean | query | Nee |  |

## Response

Retourneert: `AggregateResponse`

## Example

[inline-code-attrs-start title = 'aggregatie Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutelauthenticatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ontcommentaar hieronder om prefix in te stellen (bijv. Bearer) voor API-sleutel, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final aggregationRequest = AggregationRequest(); // AggregationRequest | 
final parentTenantId = parentTenantId_example; // String | 
final includeStats = true; // bool | 

try {
    final result = api_instance.aggregate(tenantId, aggregationRequest, AggregateOptions(parentTenantId: parentTenantId, includeStats: includeStats));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->aggregate: $e\n');
}
[inline-code-end]