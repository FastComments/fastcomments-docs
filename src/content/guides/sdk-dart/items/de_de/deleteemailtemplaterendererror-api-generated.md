## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| errorId | string | path | Yes |  |

## Antwort

Rückgabe: `APIEmptyResponse`

## Beispiel

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurieren Sie die API-Schlüssel-Authentifizierung: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Kommentar entfernen, um Präfix (z.B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
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