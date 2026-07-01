## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|--------------|----------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nein |  |
| urlId | string | query | Nein |  |
| fromCommentId | string | query | Nein |  |
| viewed | boolean | query | Nein |  |
| type | string | query | Nein |  |

## Antwort

Rückgabe: `GetNotificationCountResponse`

## Beispiel

[inline-code-attrs-start title = 'getNotificationCount Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// unten auskommentieren, um Präfix (z. B. Bearer) für API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final urlId = urlId_example; // String | 
final fromCommentId = fromCommentId_example; // String | 
final viewed = true; // bool | 
final type = type_example; // String | 

try {
    final result = api_instance.getNotificationCount(tenantId, GetNotificationCountOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getNotificationCount: $e\n');
}
[inline-code-end]