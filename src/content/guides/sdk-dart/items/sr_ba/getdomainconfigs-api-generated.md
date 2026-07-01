## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|-------|-----|----------|----------|------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: `GetDomainConfigsResponse`

## Primjer

[inline-code-attrs-start title = 'getDomainConfigs Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 

try {
    final result = api_instance.getDomainConfigs(tenantId);
    print(result);
} catch (e) {
    print('Izuzetak prilikom poziva DefaultApi->getDomainConfigs: $e\n');
}
[inline-code-end]

---