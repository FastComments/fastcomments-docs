## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| broadcastId | string | query | Da |  |
| sessionId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Response

Vraća: `SaveCommentsResponseWithPresence`

## Example

[inline-code-attrs-start title = 'createCommentPublic Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
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