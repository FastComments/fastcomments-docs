## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Antwort

Rückgabe: `APICreateUserBadgeResponse`

## Beispiel

[inline-code-attrs-start title = 'createUserBadge Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// unten auskommentieren, um Präfix (z.B. Bearer) für API-Schlüssel festzulegen, falls benötigt
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createUserBadgeParams = CreateUserBadgeParams(); // CreateUserBadgeParams | 

try {
    final result = api_instance.createUserBadge(tenantId, createUserBadgeParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createUserBadge: $e\n');
}
[inline-code-end]

---