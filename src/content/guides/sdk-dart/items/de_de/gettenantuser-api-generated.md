## Parameter

| Name   | Typ    | Ort    | Erforderlich | Beschreibung |
|--------|--------|--------|--------------|--------------|
| tenantId | string | query | Ja |  |
| id     | string | path   | Ja |  |

## Antwort

Rückgabe: `GetTenantUserResponse`

## Beispiel

[inline-code-attrs-start title = 'getTenantUser Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüsselautorisation konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// entferne den Kommentar unten, um das Präfix (z.B. Bearer) für den API-Schlüssel einzurichten, falls nötig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getTenantUser(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantUser: $e\n');
}
[inline-code-end]