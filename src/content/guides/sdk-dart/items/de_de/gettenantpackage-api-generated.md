## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|--------------|--------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Rückgabe: `GetTenantPackageResponse`

## Beispiel

[inline-code-attrs-start title = 'getTenantPackage Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Kommentar unten entfernen, um Präfix (z. B. Bearer) für API-Schlüssel festzulegen, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getTenantPackage(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantPackage: $e\n');
}
[inline-code-end]

---