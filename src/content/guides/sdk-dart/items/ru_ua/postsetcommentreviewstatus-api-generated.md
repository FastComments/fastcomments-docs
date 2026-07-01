## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| reviewed | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Ответ

Returns: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'postSetCommentReviewStatus Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final reviewed = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentReviewStatus(tenantId, commentId, PostSetCommentReviewStatusOptions(reviewed: reviewed, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentReviewStatus: $e\n');
}
[inline-code-end]