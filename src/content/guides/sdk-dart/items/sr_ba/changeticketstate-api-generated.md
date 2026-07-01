## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odgovor

Returns: `ChangeTicketStateResponse`

## Primer

[inline-code-attrs-start title = 'changeTicketState Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentarišite ispod za postavljanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final id = id_example; // String | 
final changeTicketStateBody = ChangeTicketStateBody(); // ChangeTicketStateBody | 

try {
    final result = api_instance.changeTicketState(tenantId, userId, id, changeTicketStateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->changeTicketState: $e\n');
}
[inline-code-end]