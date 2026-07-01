## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |
| approved | boolean | query | Ні |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `SetCommentApprovedResponse`

## Приклад

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final approved = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentApprovalStatus(tenantId, commentId, PostSetCommentApprovalStatusOptions(approved: approved, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentApprovalStatus: $e\n');
}
[inline-code-end]

---