Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Използва се от уиджетa за коментари, за да обогати потребителите, които току‑що се появиха чрез събитие за присъствие.  
Без контекст на страница: поверителността се прилага еднородно (частните профили се маскират).

## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | UserIds, разделени със запетая. |

## Response

Returns: `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'getUsersInfo Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | UserIds, разделени със запетая.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]