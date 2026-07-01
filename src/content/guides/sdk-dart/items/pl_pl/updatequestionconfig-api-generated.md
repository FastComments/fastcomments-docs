## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |

## Odpowiedź

Returns: `APIEmptyResponse`

## Przykład

[inline-code-attrs-start title = 'updateQuestionConfig Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebne
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateQuestionConfigBody = UpdateQuestionConfigBody(); // UpdateQuestionConfigBody | 

try {
    final result = api_instance.updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
    print(result);
} catch (e) {
    print('Wyjątek podczas wywoływania DefaultApi->updateQuestionConfig: $e\n');
}
[inline-code-end]