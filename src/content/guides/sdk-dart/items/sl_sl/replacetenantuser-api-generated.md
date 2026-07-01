## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| updateComments | string | query | Ne |  |

## Odgovor

Returns: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'replaceTenantUser Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Odkomenirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final replaceTenantUserBody = ReplaceTenantUserBody(); // ReplaceTenantUserBody | 
final updateComments = updateComments_example; // String | 

try {
    final result = api_instance.replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->replaceTenantUser: $e\n');
}
[inline-code-end]