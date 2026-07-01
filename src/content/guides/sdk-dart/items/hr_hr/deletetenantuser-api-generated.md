## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| deleteComments | string | query | Ne |  |
| commentDeleteMode | string | query | Ne |  |

## Odgovor

Vraća: `APIEmptyResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer deleteTenantUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final deleteComments = deleteComments_example; // String | 
final commentDeleteMode = commentDeleteMode_example; // String | 

try {
    final result = api_instance.deleteTenantUser(tenantId, id, DeleteTenantUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTenantUser: $e\n');
}
[inline-code-end]