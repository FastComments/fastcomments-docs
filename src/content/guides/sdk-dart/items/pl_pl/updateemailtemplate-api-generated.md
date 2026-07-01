## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odpowiedź

Zwraca: `APIEmptyResponse`

## Przykład

[inline-code-attrs-start title = 'updateEmailTemplate Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebny
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateEmailTemplateBody = UpdateEmailTemplateBody(); // UpdateEmailTemplateBody | 

try {
    final result = api_instance.updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
    print(result);
} catch (e) {
    print('Wyjątek podczas wywoływania DefaultApi->updateEmailTemplate: $e\n');
}
[inline-code-end]