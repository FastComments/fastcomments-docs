## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Odgovor

Vraća: `GetTicketResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer getTicket'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentirajte sljedeće za postavljanje prefiksa (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.getTicket(tenantId, id, userId);
    print(result);
} catch (e) {
    print('Izuzetak prilikom poziva DefaultApi->getTicket: $e\n');
}
[inline-code-end]