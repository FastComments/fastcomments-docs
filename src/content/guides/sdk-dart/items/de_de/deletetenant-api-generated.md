## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| sure | string | query | Nein |  |

## Antwort

Rückgabe: `APIEmptyResponse`

## Beispiel

[inline-code-attrs-start title = 'deleteTenant Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// unten auskommentieren, um Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final sure = sure_example; // String | 

try {
    final result = api_instance.deleteTenant(tenantId, id, sure);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTenant: $e\n');
}
[inline-code-end]

---