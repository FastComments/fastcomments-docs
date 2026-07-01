## Parameters

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Response

Vrne: `PatchPageAPIResponse`

## Example

[inline-code-attrs-start title = 'patchPage Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte avtorizacijo ključev API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za ključ API, če je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPIPageData = UpdateAPIPageData(); // UpdateAPIPageData | 

try {
    final result = api_instance.patchPage(tenantId, id, updateAPIPageData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchPage: $e\n');
}
[inline-code-end]