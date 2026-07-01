Масова інформація про користувачів для орендаря. За заданими userIds повертає відображувану інформацію з User / SSOUser.  
Використовується віджетом коментарів для збагачення користувачів, які щойно з’явились через подію присутності.  
Немає контексту сторінки: конфіденційність застосовується однорідно (приватні профілі маскуються).

## Parameters

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| ids | string | query | Так | Кома-розділені userIds. |

## Response

Повертає: `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'Приклад getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Кома-розділені userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]