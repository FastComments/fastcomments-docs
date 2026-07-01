## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| deleteComments | boolean | query | Ne |  |
| commentDeleteMode | string | query | Ne |  |

## Odgovor

Vrne: `DeleteSSOUserAPIResponse`

## Primer

[inline-code-attrs-start title = 'Primer deleteSSOUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final deleteComments = true; // bool | 
final commentDeleteMode = commentDeleteMode_example; // String | 

try {
    final result = api_instance.deleteSSOUser(tenantId, id, DeleteSSOUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteSSOUser: $e\n');
}
[inline-code-end]