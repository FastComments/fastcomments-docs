## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Odgovor

Vraća: `PatchDomainConfigResponse`

## Primer

[inline-code-attrs-start title = 'patchDomainConfig Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domainToUpdate = domainToUpdate_example; // String | 
final patchDomainConfigParams = PatchDomainConfigParams(); // PatchDomainConfigParams | 

try {
    final result = api_instance.patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchDomainConfig: $e\n');
}
[inline-code-end]