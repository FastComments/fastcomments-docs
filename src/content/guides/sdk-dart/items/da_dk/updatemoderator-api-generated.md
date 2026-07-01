## Parametre

| Navn | Type | Placering | Krævet | Beskrivelse |
|------|------|-----------|--------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Svar

Returnerer: `APIEmptyResponse`

## Eksempel

[inline-code-attrs-start title = 'updateModerator Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurér API-nøglegodkendelse: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// fjern kommentaren nedenfor for at opsætte præfiks (fx Bearer) for API-nøglen, hvis nødvendigt
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateModeratorBody = UpdateModeratorBody(); // UpdateModeratorBody | 

try {
    final result = api_instance.updateModerator(tenantId, id, updateModeratorBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateModerator: $e\n');
}
[inline-code-end]