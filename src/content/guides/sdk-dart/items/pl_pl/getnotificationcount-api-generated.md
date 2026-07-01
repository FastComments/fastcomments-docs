## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | query | Tak |  |
| userId | string | query | Nie |  |
| urlId | string | query | Nie |  |
| fromCommentId | string | query | Nie |  |
| viewed | boolean | query | Nie |  |
| type | string | query | Nie |  |

## Odpowiedź

Zwraca: `GetNotificationCountResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład getNotificationCount'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli jest potrzebny
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final urlId = urlId_example; // String | 
final fromCommentId = fromCommentId_example; // String | 
final viewed = true; // bool | 
final type = type_example; // String | 

try {
    final result = api_instance.getNotificationCount(tenantId, GetNotificationCountOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getNotificationCount: $e\n');
}
[inline-code-end]