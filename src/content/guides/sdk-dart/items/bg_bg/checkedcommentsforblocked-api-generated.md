## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| commentIds | string | query | Да | Списък с идентификатори на коментари, разделени със запетая. |
| sso | string | query | Не |  |

## Отговор

Връща: `CheckBlockedCommentsResponse`

## Пример

[inline-code-attrs-start title = 'Пример за checkedCommentsForBlocked'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Списък с идентификатори на коментари, разделени със запетая.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]