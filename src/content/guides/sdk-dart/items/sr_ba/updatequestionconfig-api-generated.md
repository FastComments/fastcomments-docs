## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'updateQuestionConfig Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
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