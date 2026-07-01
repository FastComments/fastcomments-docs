## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Returns: `APIEmptyResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio replaceTenantPackage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configura l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// decommenta qui sotto per impostare il prefisso (es. Bearer) per la chiave API, se necessario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final replaceTenantPackageBody = ReplaceTenantPackageBody(); // ReplaceTenantPackageBody | 

try {
    final result = api_instance.replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->replaceTenantPackage: $e\n');
}
[inline-code-end]

---