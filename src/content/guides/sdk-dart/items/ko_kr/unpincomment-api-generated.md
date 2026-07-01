---
## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | Yes |  |
| sso | string | query | No |  |

## 응답

Returns: `ChangeCommentPinStatusResponse`

## 예시

[inline-code-attrs-start title = 'unPinComment 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.unPinComment(tenantId, commentId, broadcastId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->unPinComment: $e\n');
}
[inline-code-end]

---