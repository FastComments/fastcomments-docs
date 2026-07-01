## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| updateComments | string | query | Nein |  |

## Antwort

Returns: `APIEmptyResponse`

## Beispiel

[inline-code-attrs-start title = 'updateTenantUser Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// unten auskommentieren, um ein Präfix (z. B. Bearer) für den API-Schlüssel festzulegen, falls erforderlich
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