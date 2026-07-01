## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| skip | number | query | Nein |  |

## Antwort

Rückgabe: `GetEmailTemplateRenderErrorsResponse`

## Beispiel

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüsselautorisierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Kommentar unten auskommentieren, um das Präfix (z.B. Bearer) für den API-Schlüssel einzurichten, falls nötig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getEmailTemplateRenderErrors(tenantId, id, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getEmailTemplateRenderErrors: $e\n');
}
[inline-code-end]