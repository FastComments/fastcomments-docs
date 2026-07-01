## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: `AddDomainConfigResponse`

## Primjer

[inline-code-attrs-start title = 'addDomainConfig Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final addDomainConfigParams = AddDomainConfigParams(); // AddDomainConfigParams | 

try {
    final result = api_instance.addDomainConfig(tenantId, addDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addDomainConfig: $e\n');
}
[inline-code-end]

---