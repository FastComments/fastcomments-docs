## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| updateComments | string | query | Nej |  |

## Svar

Returnerer: `APIEmptyResponse`

## Eksempel

[inline-code-attrs-start title = 'updateTenantUser Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøglegodkendelse: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// fjern kommentaren nedenfor for at konfigurere præfiks (f.eks. Bearer) for API-nøgle, hvis nødvendigt
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantUserBody = UpdateTenantUserBody(); // UpdateTenantUserBody | 
final updateComments = updateComments_example; // String | 

try {
    final result = api_instance.updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenantUser: $e\n');
}
[inline-code-end]