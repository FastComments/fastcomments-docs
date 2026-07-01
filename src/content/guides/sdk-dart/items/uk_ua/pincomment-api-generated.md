## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| broadcastId | string | query | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `ChangeCommentPinStatusResponse`

## Приклад

[inline-code-attrs-start title = 'pinComment Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.pinComment(tenantId, commentId, broadcastId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->pinComment: $e\n');
}
[inline-code-end]