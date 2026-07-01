## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odpowiedź

Zwraca: `APIEmptySuccessResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład updateUserBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebny
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateUserBadgeParams = UpdateUserBadgeParams(); // UpdateUserBadgeParams | 

try {
    final result = api_instance.updateUserBadge(tenantId, id, updateUserBadgeParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateUserBadge: $e\n');
}
[inline-code-end]