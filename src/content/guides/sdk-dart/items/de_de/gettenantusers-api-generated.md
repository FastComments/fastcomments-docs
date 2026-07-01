## Parameter

| Name | Type | Location | Required | Beschreibung |
|------|------|----------|----------|---------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Antwort

Rückgabe: `GetTenantUsersResponse`

## Beispiel

[inline-code-attrs-start title = 'getTenantUsers Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Autorisierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// entfernen Sie das Kommentarzeichen unten, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantUsers(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantUsers: $e\n');
}
[inline-code-end]