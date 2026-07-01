## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Odpowiedź

Zwraca: `GetTenantUsersResponse`

## Przykład

[inline-code-attrs-start title = 'getTenantUsers Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, w razie potrzeby
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantUsers(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantUsers: $e\n');
}
[inline-code-end]

---