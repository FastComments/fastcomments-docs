Массовая информация о пользователях для тенанта. По заданным userIds возвращает отображаемую информацию из User / SSOUser.
Используется виджетом комментариев для обогащения пользователей, появившихся через событие присутствия.
Без контекста страницы: приватность соблюдается единообразно (приватные профили маскируются).

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| ids | string | query | Да | userIds, разделённые запятыми. |

## Ответ

Возвращает: `PageUsersInfoResponse`

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | userIds, разделённые запятыми.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]