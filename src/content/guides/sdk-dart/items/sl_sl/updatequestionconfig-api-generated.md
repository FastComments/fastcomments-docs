## Parameters

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odgovor

Vrne: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'updateQuestionConfig Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Nastavite avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateQuestionConfigBody = UpdateQuestionConfigBody(); // UpdateQuestionConfigBody | 

try {
    final result = api_instance.updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateQuestionConfig: $e\n');
}
[inline-code-end]