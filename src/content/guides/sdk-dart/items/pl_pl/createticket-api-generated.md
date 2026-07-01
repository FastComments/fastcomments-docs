## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |

## Response

Zwraca: `CreateTicketResponse`

## Example

[inline-code-attrs-start title = 'createTicket Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final createTicketBody = CreateTicketBody(); // CreateTicketBody | 

try {
    final result = api_instance.createTicket(tenantId, userId, createTicketBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTicket: $e\n');
}
[inline-code-end]