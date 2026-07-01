Aggrega i documenti raggruppandoli (se groupBy è fornito) e applicando più operazioni.  
Sono supportate diverse operazioni (ad es. sum, countDistinct, avg, ecc.).

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## Risposta

Restituisce: `AggregateResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregazione'; type = ''; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import 'package:fastcomments_dart/api.dart';  
// TODO Configura l'autorizzazione della chiave API: api_key  
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';  
// decommenta qui sotto per impostare il prefisso (ad es. Bearer) per la chiave API, se necessario  
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