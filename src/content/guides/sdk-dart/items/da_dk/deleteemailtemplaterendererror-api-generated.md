## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| errorId | string | path | Yes |  |

## Svar

Returnerer: `APIEmptyResponse`

## Eksempel

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøgle autorisation: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// fjern kommentaren nedenfor for at opsætte præfiks (fx Bearer) for API-nøgle, hvis nødvendigt
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final errorId = errorId_example; // String | 

try {
    final result = api_instance.deleteEmailTemplateRenderError(tenantId, id, errorId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteEmailTemplateRenderError: $e\n');
}
[inline-code-end]