## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | Abfrage | Ja |  |
| email | string | Pfad | Ja |  |

## Antwort

Rückgabe: `GetSSOUserByEmailAPIResponse`

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getSSOUserByEmail'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entfernen Sie das Kommentarzeichen unten, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final email = email_example; // String | 

try {
    final result = api_instance.getSSOUserByEmail(tenantId, email);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUserByEmail: $e\n');
}
[inline-code-end]