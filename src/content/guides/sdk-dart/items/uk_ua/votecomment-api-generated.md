## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| urlId | string | query | Так |  |
| broadcastId | string | query | Так |  |
| sessionId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Returns: `VoteResponse`

## Приклад

[inline-code-attrs-start title = 'voteComment Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final urlId = urlId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final voteBodyParams = VoteBodyParams(); // VoteBodyParams | 
final sessionId = sessionId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, VoteCommentOptions(sessionId: sessionId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->voteComment: $e\n');
}
[inline-code-end]

---