## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Antwort

Rückgabe: `APIEmptyResponse`

## Beispiel

[inline-code-attrs-start title = 'replaceTenantPackage Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurieren Sie die API-Schlüssel-Authentifizierung: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entfernen Sie den Kommentar unten, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls nötig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final replaceTenantPackageBody = ReplaceTenantPackageBody(); // ReplaceTenantPackageBody | 

try {
    final result = api_instance.replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->replaceTenantPackage: $e\n');
}
[inline-code-end]