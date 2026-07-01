## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | query | Yes |  |

## Odpowiedź

Zwraca: `CreateTenantUserResponse`

## Przykład

[inline-code-attrs-start title = 'createTenantUser Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, w razie potrzeby
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantUserBody = CreateTenantUserBody(); // CreateTenantUserBody | 

try {
    final result = api_instance.createTenantUser(tenantId, createTenantUserBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenantUser: $e\n');
}
[inline-code-end]

---