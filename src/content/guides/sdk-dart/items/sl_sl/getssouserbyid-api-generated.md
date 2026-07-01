## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odziv

Vrne: `GetSSOUserByIdAPIResponse`

## Primer

[inline-code-attrs-start title = 'getSSOUserById Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfiguriraj avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Odkomentiraj spodaj, da nastaviš predpono (npr. Bearer) za API ključ, po potrebi

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getSSOUserById(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUserById: $e\n');
}
[inline-code-end]