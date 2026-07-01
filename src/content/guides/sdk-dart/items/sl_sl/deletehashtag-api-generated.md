## Parameters

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| tag | string | path | Da |  |

## Response

Vrne: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'deleteHashTag Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO konfigurirajte avtentikacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final deleteHashTagRequestBody = DeleteHashTagRequestBody(); // DeleteHashTagRequestBody | 

try {
    final result = api_instance.deleteHashTag(tenantId, tag, deleteHashTagRequestBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteHashTag: $e\n');
}
[inline-code-end]