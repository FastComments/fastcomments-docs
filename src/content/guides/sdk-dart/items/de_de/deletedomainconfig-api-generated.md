## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|---------------|
| tenantId | string | query | Ja |  |
| domain | string | path | Ja |  |

## Antwort

Rückgabe: `DeleteDomainConfigResponse`

## Beispiel

[inline-code-attrs-start title = 'deleteDomainConfig Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entfernen Sie das Kommentarzeichen unten, um das Präfix (z.B. Bearer) für den API-Schlüssel festzulegen, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domain = domain_example; // String | 

try {
    final result = api_instance.deleteDomainConfig(tenantId, domain);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteDomainConfig: $e\n');
}
[inline-code-end]

---