## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | boolean | query | No |  |

## Svar

Returnerer: `PutSSOUserAPIResponse`

## Eksempel

[inline-code-attrs-start title = 'putSSOUser Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøgleautorisering: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// fjern kommentar nedenfor for at konfigurere præfiks (f.eks. Bearer) for API-nøgle, hvis nødvendigt
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPISSOUserData = UpdateAPISSOUserData(); // UpdateAPISSOUserData | 
final updateComments = true; // bool | 

try {
    final result = api_instance.putSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->putSSOUser: $e\n');
}
[inline-code-end]