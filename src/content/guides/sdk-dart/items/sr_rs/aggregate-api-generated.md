Agregira dokumente grupisanjem (ako je naveden groupBy) i primenom više operacija.  
Različite operacije (npr. sum, countDistinct, avg, itd.) su podržane.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| parentTenantId | string | query | Ne |  |
| includeStats | boolean | query | Ne |  |

## Response

Vraća: `AggregateResponse`

## Example

[inline-code-attrs-start title = 'Primer agregacije'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуришите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite dole da postavite prefiks (npr. Bearer) za API ključ, po potrebi
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