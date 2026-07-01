## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |

## Antwort

Rückgabe: `FlagCommentResponse`

## Beispiel

[inline-code-attrs-start title = 'flagComment Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurieren Sie die API-Schlüssel-Authentifizierung: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Kommentieren Sie die Zeile unten aus, um ein Präfix (z. B. Bearer) für den API‑Schlüssel festzulegen, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.flagComment(tenantId, id, FlagCommentOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->flagComment: $e\n');
}
[inline-code-end]