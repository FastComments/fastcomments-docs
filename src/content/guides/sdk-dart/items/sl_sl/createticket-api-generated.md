## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |

## Odziv

Vrne: `CreateTicketResponse`

## Primer

[inline-code-attrs-start title = 'createTicket Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Nastavite avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, po potrebi
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