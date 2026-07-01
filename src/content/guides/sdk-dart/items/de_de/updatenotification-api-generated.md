## Parameter

| Name    | Typ    | Ort   | Erforderlich | Beschreibung |
|---------|--------|-------|--------------|--------------|
| tenantId | string | query | Ja           |  |
| id       | string | path  | Ja           |  |
| userId   | string | query | Nein         |  |

## Antwort

Rückgabe: `APIEmptyResponse`

## Beispiel

[inline-code-attrs-start title = 'updateNotification Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API‑Schlüsselautorisierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entkommentieren Sie unten, um das Präfix (z. B. Bearer) für den API‑Schlüssel festzulegen, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateNotificationBody = UpdateNotificationBody(); // UpdateNotificationBody | 
final userId = userId_example; // String | 

try {
    final result = api_instance.updateNotification(tenantId, id, updateNotificationBody, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateNotification: $e\n');
}
[inline-code-end]