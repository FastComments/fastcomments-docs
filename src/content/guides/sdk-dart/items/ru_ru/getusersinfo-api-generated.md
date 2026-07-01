Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Used by the comment widget to enrich users that just appeared via a presence event.  
No page context: privacy is enforced uniformly (private profiles are masked).

## Parameters

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Идентификаторы пользователей, разделённые запятыми. |

## Response

Возвращает: `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Идентификаторы пользователей, разделённые запятыми.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]