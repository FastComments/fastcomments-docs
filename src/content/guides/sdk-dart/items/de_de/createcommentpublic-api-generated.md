## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| broadcastId | string | query | Ja |  |
| sessionId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Returns: `SaveCommentsResponseWithPresence`

## Beispiel

[inline-code-attrs-start title = 'createCommentPublic Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final commentData = CommentData(); // CommentData | 
final sessionId = sessionId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.createCommentPublic(tenantId, urlId, broadcastId, commentData, CreateCommentPublicOptions(sessionId: sessionId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createCommentPublic: $e\n');
}
[inline-code-end]