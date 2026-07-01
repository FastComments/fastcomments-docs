## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| commentId | string | path | Evet |  |
| broadcastId | string | query | Evet |  |
| editKey | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Response

Döndürür: `PublicAPISetCommentTextResponse`

## Example

[inline-code-attrs-start title = 'setCommentText Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final commentTextUpdateRequest = CommentTextUpdateRequest(); // CommentTextUpdateRequest | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, SetCommentTextOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->setCommentText: $e\n');
}
[inline-code-end]