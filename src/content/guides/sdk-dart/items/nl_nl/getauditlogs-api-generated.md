## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## Response

Retourneert: `GetAuditLogsResponse`

## Example

[inline-code-attrs-start title = 'getAuditLogs Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutautorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// verwijder commentaar hieronder om prefix (bijv. Bearer) voor API-sleutel in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 
final order = ; // SORTDIR | 
final after = 1.2; // double | 
final before = 1.2; // double | 

try {
    final result = api_instance.getAuditLogs(tenantId, GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getAuditLogs: $e\n');
}
[inline-code-end]