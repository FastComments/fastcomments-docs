## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: `BulkCreateHashTagsResponse`

## Przykład

[inline-code-attrs-start title = 'addHashTagsBulk Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację klucza API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebny
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String |
final bulkCreateHashTagsBody = BulkCreateHashTagsBody(); // BulkCreateHashTagsBody |

try {
    final result = api_instance.addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addHashTagsBulk: $e\n');
}
[inline-code-end]