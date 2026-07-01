## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: `BulkCreateHashTagsResponse`

## Primjer

[inline-code-attrs-start title = 'addHashTagsBulk Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO: konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
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