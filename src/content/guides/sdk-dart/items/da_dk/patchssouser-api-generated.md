## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | boolean | query | No |  |

## Svar

Returns: `PatchSSOUserAPIResponse`

## Eksempel

[inline-code-attrs-start title = 'patchSSOUser Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøglegodkendelse: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Fjern kommentaren nedenfor for at opsætte præfiks (fx Bearer) for API-nøglen, hvis nødvendigt
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String |
final id = id_example; // String |
final updateAPISSOUserData = UpdateAPISSOUserData(); // UpdateAPISSOUserData |
final updateComments = true; // bool |

try {
    final result = api_instance.patchSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchSSOUser: $e\n');
}
[inline-code-end]