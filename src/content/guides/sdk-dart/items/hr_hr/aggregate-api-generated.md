Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## Odgovor

Vraća: `AggregateResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer agregacije'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
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