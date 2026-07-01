## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| meta | string | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: `GetTenantsResponse`

## Eksempel

[inline-code-attrs-start title = 'getTenants Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøglegodkendelse: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// fjern kommentar nedenfor for at indstille præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
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