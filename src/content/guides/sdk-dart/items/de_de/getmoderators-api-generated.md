## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|--------------|-----------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nein |  |

## Antwort

Returns: `GetModeratorsResponse`

## Beispiel

[inline-code-attrs-start title = 'getModerators Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurieren Sie die Autorisierung des API-Schlüssels: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entfernen Sie den Kommentar unten, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls nötig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getModerators(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getModerators: $e\n');
}
[inline-code-end]

---