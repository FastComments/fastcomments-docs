## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| userId | string | query | Nie |  |
| state | number | query | Nie |  |
| skip | number | query | Nie |  |
| limit | number | query | Nie |  |

## Odpowiedź

Zwraca: `GetTicketsResponse`

## Przykład

[inline-code-attrs-start title = 'getTickets Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebny
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final state = 1.2; // double | 
final skip = 1.2; // double | 
final limit = 1.2; // double | 

try {
    final result = api_instance.getTickets(tenantId, GetTicketsOptions(userId: userId, state: state, skip: skip, limit: limit));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTickets: $e\n');
}
[inline-code-end]

---