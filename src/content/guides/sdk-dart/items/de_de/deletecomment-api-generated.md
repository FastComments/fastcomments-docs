## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|---------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| contextUserId | string | query | Nein |  |
| isLive | boolean | query | Nein |  |

## Antwort

Rückgabe: `DeleteCommentResult`

## Beispiel

[inline-code-attrs-start title = 'deleteComment Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entfernen Sie das Kommentarzeichen unten, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final contextUserId = contextUserId_example; // String | 
final isLive = true; // bool | 

try {
    final result = api_instance.deleteComment(tenantId, id, DeleteCommentOptions(contextUserId: contextUserId, isLive: isLive));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteComment: $e\n');
}
[inline-code-end]