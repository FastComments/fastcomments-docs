## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odziv

Returns: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'Primer updateQuestionResult'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO konfigurirajte avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateQuestionResultBody = UpdateQuestionResultBody(); // UpdateQuestionResultBody | 

try {
    final result = api_instance.updateQuestionResult(tenantId, id, updateQuestionResultBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateQuestionResult: $e\n');
}
[inline-code-end]