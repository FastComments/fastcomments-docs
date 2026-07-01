## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vrne: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'deleteEmailTemplate Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Nastavite avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteEmailTemplate(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteEmailTemplate: $e\n');
}
[inline-code-end]