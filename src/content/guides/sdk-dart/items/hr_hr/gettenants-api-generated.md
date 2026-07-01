## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## Odgovor

Vraća: `GetTenantsResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer getTenants'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO: Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final meta = meta_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenants(tenantId, GetTenantsOptions(meta: meta, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenants: $e\n');
}
[inline-code-end]