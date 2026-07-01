## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Antwort

Returns: `APIEmptySuccessResponse`

## Beispiel

[inline-code-attrs-start title = 'updateUserBadge Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurieren Sie die API-Schlüssel-Authentifizierung: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// entkommentieren Sie unten, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateUserBadgeParams = UpdateUserBadgeParams(); // UpdateUserBadgeParams | 

try {
    final result = api_instance.updateUserBadge(tenantId, id, updateUserBadgeParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateUserBadge: $e\n');
}
[inline-code-end]

---