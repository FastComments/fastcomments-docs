## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |

## Risposta

Returns: `CreateTenantPackageResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio createTenantPackage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configura l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantPackageBody = CreateTenantPackageBody(); // CreateTenantPackageBody | 

try {
    final result = api_instance.createTenantPackage(tenantId, createTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenantPackage: $e\n');
}
[inline-code-end]

---